use glam::Vec2;
use std::rc::Rc;

use crate::ship::{Ship, ShipConfig};
use crate::simulation::Simulation;

const GOLDEN_ANGLE: f32 = 2.399_963_23;

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
            vision_range: 300.0,
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
            let x = f32::sqrt(n) * f32::cos(n * GOLDEN_ANGLE) * swarm_config.scale;
            let y = f32::sqrt(n) * f32::sin(n * GOLDEN_ANGLE) * swarm_config.scale;
            let ship = Ship::spawn(Vec2::new(x, y), Rc::clone(&ship_config));
            ships.push((ship, Vec2::new(x, y)));
        }

        Swarm {
            ships,
            target_pos: pos,
            center: pos,
            direction: 0.0,
            config: swarm_config,
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

    pub fn fight(&mut self) {
        for (ship, _) in &mut self.ships {
            ship.fight();
        }
    }

    pub fn movement(&mut self) {
        for (ship, _) in &mut self.ships {
            ship.movement();
        }
    }

    pub fn finalize(&mut self) {
        self.ships.retain(|(ship, _)| ship.health > 0);
        self.center = self.ships.iter().map(|(s, _)| s.pos).sum::<Vec2>() / self.ships.len() as f32;
    }

    pub fn num_ships(&self) -> u32 {
        self.ships.len() as u32
    }

    /// Make decisions based on the current simulation state
    pub fn decide(&self, sim: &Simulation, self_idx: usize) -> Option<SwarmDecision> {
        let nearby_swarms = sim.get_swarms_in_range(self_idx);
        let bounds = sim.bounds();

        let mut target: Option<Vec2> = None;
        let mut is_threat = false;

        for (swarm, _dist) in nearby_swarms {
            if swarm.num_ships() >= self.num_ships() {
                target = Some(swarm.center);
                is_threat = true;
                break;
            } else if target.is_none() {
                target = Some(swarm.center);
            }
        }

        target.map(|t| {
            let final_target = if is_threat {
                const WALL_MARGIN: f32 = 5.0;
                const FLEE_DISTANCE: f32 = 400.0;
                const WALL_HIT_THRESHOLD: f32 = 0.5;
                const FLEE_WEIGHT: f32 = 0.5;
                const SLIDE_WEIGHT: f32 = 0.5;

                let flee_dir = (self.center - t).normalize_or_zero();
                let naive_target = self.center + flee_dir * FLEE_DISTANCE;
                let clamped_target = bounds.clamp_with_margin(naive_target, WALL_MARGIN);

                // Check if we'd hit a wall (clamped target is much closer than intended)
                let actual_dist = clamped_target.distance(self.center);

                if actual_dist < FLEE_DISTANCE * WALL_HIT_THRESHOLD {
                    // We'd hit a wall - steer to slide along it
                    // Find perpendicular directions to flee_dir
                    let perp1 = Vec2::new(-flee_dir.y, flee_dir.x);
                    let perp2 = Vec2::new(flee_dir.y, -flee_dir.x);

                    // Try both perpendicular directions, pick the one that gives more distance
                    let target1 =
                        bounds.clamp_with_margin(self.center + perp1 * FLEE_DISTANCE, WALL_MARGIN);
                    let target2 =
                        bounds.clamp_with_margin(self.center + perp2 * FLEE_DISTANCE, WALL_MARGIN);

                    let dist1 = target1.distance(self.center);
                    let dist2 = target2.distance(self.center);

                    // Blend: mostly perpendicular (slide along wall) with some flee component
                    let best_perp = if dist1 > dist2 { perp1 } else { perp2 };
                    let blended_dir =
                        (flee_dir * FLEE_WEIGHT + best_perp * SLIDE_WEIGHT).normalize_or_zero();

                    bounds.clamp_with_margin(self.center + blended_dir * FLEE_DISTANCE, WALL_MARGIN)
                } else {
                    clamped_target
                }
            } else {
                t
            };
            SwarmDecision {
                target: final_target,
                is_threat,
            }
        })
    }

    /// Apply a decision to this swarm
    pub fn apply_decision(&mut self, decision: &SwarmDecision) {
        self.set_target(decision.target);
    }
}
