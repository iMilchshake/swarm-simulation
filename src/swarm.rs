use std::collections::HashMap;

use glam::Vec2;
use std::rc::Rc;

use crate::repulsion::RepulsionMap;
use crate::ship::{Ship, ShipConfig, ShipId};
use crate::simulation::Simulation;

const GOLDEN_ANGLE: f32 = 2.399_963_1;

#[derive(Clone)]
pub struct SwarmConfig {
    /// maximum number of ships in a swarm
    pub max_ships: u32,

    /// swarm position distribution scale
    pub scale: f32,

    /// maximum vision range to see other swarms
    pub vision_range: f32,
}

impl Default for SwarmConfig {
    fn default() -> Self {
        SwarmConfig {
            max_ships: 30,
            scale: 10.0,
            vision_range: 500.0,
        }
    }
}

/// Swarm consisting of multiple ships.
/// Ships that are part of the swarm are assigned a position releative to
/// the Swarms target position.
/// The swarms own position is the average position of all ships
pub struct Swarm {
    /// keeps track of all ships and their **relative** position to swarm's target position
    pub ships: Vec<(Ship, Vec2)>,
    pub target_pos: Vec2,
    pub direction: f32,
    pub center: Vec2,
    pub config: Rc<SwarmConfig>,
    /// Track current movement velocity for momentum penalty
    pub velocity: Vec2,
    prev_center: Vec2,
}

#[derive(Debug)]
pub struct SwarmDecision {
    pub target: Vec2,
    pub is_threat: bool,
}

impl Swarm {
    /// Spawn a new swarm with n ships at a given location
    pub fn spawn(
        pos: Vec2,
        num_ships: u32,
        swarm_config: Rc<SwarmConfig>,
        ship_config: Rc<ShipConfig>,
    ) -> Swarm {
        let mut ships = Vec::new();

        // spawn ships in spiral layout using golden angle (sunflower structure)
        for ship_idx in 0..num_ships {
            let n = ship_idx as f32;
            let x_local = f32::sqrt(n) * f32::cos(n * GOLDEN_ANGLE) * swarm_config.scale;
            let y_local = f32::sqrt(n) * f32::sin(n * GOLDEN_ANGLE) * swarm_config.scale;
            let ship = Ship::spawn(
                Vec2::new(pos.x + x_local, pos.y + y_local),
                Rc::clone(&ship_config),
            );
            ships.push((ship, Vec2::new(x_local, y_local)));
        }

        Swarm {
            ships,
            target_pos: pos,
            center: pos,
            direction: 0.0,
            config: swarm_config,
            velocity: Vec2::ZERO,
            prev_center: pos,
        }
    }

    pub fn set_target(&mut self, pos: Vec2) {
        let to_target = pos - self.center;

        // rotate all relative positions
        let new_direction = to_target.y.atan2(to_target.x);
        let rotation = Vec2::from_angle(new_direction - self.direction);
        for (_, relative_pos) in &mut self.ships {
            *relative_pos = relative_pos.rotate(rotation);
        }
        self.direction = new_direction;

        // update ship global target positions
        self.target_pos = pos;
        for (ship, relative_pos) in &mut self.ships {
            ship.set_target(pos + *relative_pos);
        }
    }

    /// Each ship locks onto the nearest enemy in aim_range, fires after a
    /// delay that scales with enemy speed. Returns IDs of enemies that were hit.
    pub fn fight(&mut self, enemies: &[&Ship]) -> Vec<ShipId> {
        let mut hits: Vec<ShipId> = Vec::new();

        // count how many of our ships already target each enemy
        let mut targeted_count: HashMap<ShipId, u32> = HashMap::new();
        for (ship, _) in self.ships.iter() {
            if let Some(target_id) = ship.lock_target {
                *targeted_count.entry(target_id).or_default() += 1;
            }
        }

        for (ship, _) in &mut self.ships {
            ship.fired_at = None;
            ship.lock_target_pos = None;

            // validate existing lock
            if let Some(target_id) = ship.lock_target {
                let target = enemies.iter().find(|enemy| enemy.id == target_id);

                let valid = target
                    .is_some_and(|target| ship.pos.distance(target.pos) <= ship.config.aim_range);

                if !valid {
                    *targeted_count.entry(target_id).or_default() =
                        targeted_count.get(&target_id).unwrap_or(&1) - 1;
                    ship.lock_target = None;
                    ship.lock_progress = 0;
                }
            }

            // progress existing lock or fire
            if let Some(target_id) = ship.lock_target {
                let target = enemies.iter().find(|enemy| enemy.id == target_id).unwrap();
                ship.lock_target_pos = Some(target.pos);
                ship.lock_progress += 1;

                let speed_ratio = target.speed() / ship.config.max_speed;
                let multiplier = 1.0 + speed_ratio * (ship.config.lock_time_factor - 1.0);
                let lock_time = (ship.config.fire_delay as f32 * multiplier) as u32;

                if ship.lock_progress >= lock_time {
                    ship.fired_at = Some(target.pos);
                    hits.push(target_id);

                    // reset lock after firing
                    *targeted_count.entry(target_id).or_default() =
                        targeted_count.get(&target_id).unwrap_or(&1) - 1;
                    ship.lock_target = None;
                    ship.lock_progress = 0;
                }
                continue;
            }

            // acquire new target: nearest enemy in range, not over-targeted
            let best = enemies
                .iter()
                .filter(|enemy| {
                    let dist = ship.pos.distance(enemy.pos);
                    let count = targeted_count.get(&enemy.id).copied().unwrap_or(0);
                    dist <= ship.config.aim_range && count < enemy.health
                })
                .min_by(|a, b| {
                    let dist_a = ship.pos.distance_squared(a.pos);
                    let dist_b = ship.pos.distance_squared(b.pos);
                    dist_a.partial_cmp(&dist_b).unwrap()
                });

            if let Some(target) = best {
                ship.lock_target = Some(target.id);
                ship.lock_progress = 0;
                ship.lock_target_pos = Some(target.pos);
                *targeted_count.entry(target.id).or_default() += 1;
            }
        }

        hits
    }

    pub fn movement(&mut self) {
        for (ship, _) in &mut self.ships {
            ship.movement();
        }
    }

    pub fn finalize(&mut self) {
        self.ships.retain(|(ship, _)| ship.health > 0);
        let new_center =
            self.ships.iter().map(|(s, _)| s.pos).sum::<Vec2>() / self.ships.len() as f32;
        self.velocity = new_center - self.prev_center;
        self.prev_center = self.center;
        self.center = new_center;
    }

    pub fn num_ships(&self) -> u32 {
        self.ships.len() as u32
    }

    /// Make decisions based on the current simulation state
    pub fn decide(&self, sim: &Simulation, self_idx: usize) -> Option<SwarmDecision> {
        // Tunable constants
        const ENEMY_SIGMA: f32 = 0.8; // ~45 degrees spread
        const WALL_SIGMA: f32 = 0.5;
        const VELOCITY_SIGMA: f32 = 1.0;
        const WALL_DETECT_RANGE: f32 = 150.0;
        const FLEE_DISTANCE: f32 = 400.0;
        const WALL_MARGIN: f32 = 50.0;
        const VELOCITY_PENALTY_STRENGTH: f32 = 0.3;
        const PREY_SIZE_DIFFERENCE: u32 = 5; // must be this much smaller to be considered prey

        let nearby_swarms = sim.get_swarms_in_range(self_idx);
        let bounds = sim.bounds();

        // Find if there are any threats and potential chase targets
        let mut chase_target: Option<Vec2> = None;
        let mut has_threat = false;

        for (swarm, _dist) in &nearby_swarms {
            if swarm.num_ships() + PREY_SIZE_DIFFERENCE >= self.num_ships() {
                has_threat = true;
            } else if chase_target.is_none() {
                chase_target = Some(swarm.center);
            }
        }

        if has_threat {
            let mut repulsion = RepulsionMap::new();

            for (enemy, dist) in &nearby_swarms {
                if enemy.num_ships() >= self.num_ships() {
                    let angle = (enemy.center - self.center).to_angle();
                    // scale strength by ship count ratio and inverse distance TODO: is this good?
                    let ship_ratio = enemy.num_ships() as f32 / self.num_ships().max(1) as f32;
                    let dist_factor = 1.0 - (dist / self.config.vision_range).min(1.0);
                    let strength = ship_ratio * dist_factor;
                    repulsion.add_repulsor(angle, strength, ENEMY_SIGMA);
                }
            }
            repulsion.add_wall_repulsion(self.center, bounds, WALL_DETECT_RANGE, WALL_SIGMA);
            repulsion.add_velocity_penalty(
                self.velocity,
                VELOCITY_PENALTY_STRENGTH,
                VELOCITY_SIGMA,
            );

            let best_angle = repulsion.best_angle();
            let flee_dir = Vec2::from_angle(best_angle);
            let target =
                bounds.clamp_with_margin(self.center + flee_dir * FLEE_DISTANCE, WALL_MARGIN);

            Some(SwarmDecision {
                target,
                is_threat: true,
            })
        } else {
            chase_target.map(|target| SwarmDecision {
                target,
                is_threat: false,
            })
        }
    }

    /// Apply a decision to this swarm
    pub fn apply_decision(&mut self, decision: &SwarmDecision) {
        self.set_target(decision.target);
    }
}
