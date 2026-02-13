use glam::Vec2;
use std::rc::Rc;

use crate::ship::{ShipConfig, ShipId};
use crate::swarm::{Swarm, SwarmConfig, SwarmDecision};

pub struct SimulationConfig {
    /// maximum number of swarms in the simulation
    pub max_swarms: u32,

    // initial number of swarms
    pub init_swarms: u32,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        SimulationConfig {
            max_swarms: 10,
            init_swarms: 2,
        }
    }
}

pub struct Bounds {
    pub min: Vec2,
    pub max: Vec2,
}

impl Bounds {
    pub fn new(width: f32, height: f32) -> Self {
        Bounds {
            min: Vec2::ZERO,
            max: Vec2::new(width, height),
        }
    }

    pub fn clamp(&self, pos: Vec2) -> Vec2 {
        pos.clamp(self.min, self.max)
    }

    pub fn clamp_with_margin(&self, pos: Vec2, margin: f32) -> Vec2 {
        pos.clamp(
            self.min + Vec2::splat(margin),
            self.max - Vec2::splat(margin),
        )
    }

    /// Returns a vector pointing away from nearby walls, with strength based on proximity.
    /// Handles corners by combining repulsion from multiple walls.
    pub fn wall_avoidance(&self, pos: Vec2, detect_range: f32) -> Vec2 {
        let left_dist = pos.x - self.min.x;
        let top_dist = pos.y - self.min.y;
        let right_dist = self.max.x - pos.x;
        let bot_dist = self.max.y - pos.y;

        let mut avoidance = Vec2::ZERO;

        // Add repulsion from each wall based on proximity
        if left_dist < detect_range {
            avoidance.x += 1.0 - (left_dist / detect_range);
        }
        if right_dist < detect_range {
            avoidance.x -= 1.0 - (right_dist / detect_range);
        }
        if top_dist < detect_range {
            avoidance.y += 1.0 - (top_dist / detect_range);
        }
        if bot_dist < detect_range {
            avoidance.y -= 1.0 - (bot_dist / detect_range);
        }

        avoidance
    }
}

pub struct Simulation {
    swarms: Vec<Swarm>,
    config: SimulationConfig,
    ship_config: Rc<ShipConfig>,
    swarm_config: Rc<SwarmConfig>,
    bounds: Bounds,
}

impl Simulation {
    pub fn new(config: SimulationConfig, bounds: Bounds) -> Simulation {
        // TODO: replace with input configs
        let ship_config = Rc::new(ShipConfig::default());
        let swarm_config = Rc::new(SwarmConfig::default());

        Simulation {
            swarms: vec![],
            config: SimulationConfig {
                max_swarms: config.max_swarms,
                init_swarms: config.init_swarms,
            },
            ship_config,
            swarm_config,
            bounds,
        }
    }

    pub fn swarms(&self) -> &[Swarm] {
        &self.swarms
    }

    pub fn bounds(&self) -> &Bounds {
        &self.bounds
    }

    /// Spawn a new swarm at the given position, returns its index
    pub fn spawn_swarm(&mut self, pos: Vec2, num_ships: u32) -> usize {
        let swarm = Swarm::spawn(
            pos,
            num_ships,
            Rc::clone(&self.swarm_config),
            Rc::clone(&self.ship_config),
        );
        self.swarms.push(swarm);
        self.swarms.len() - 1
    }

    /// Get swarm indices, positions, and distances within vision range, sorted by distance
    pub fn get_swarms_in_range(&self, swarm_idx: usize) -> Vec<(&Swarm, f32)> {
        let swarm = &self.swarms[swarm_idx];
        let range_sq = self.swarm_config.vision_range * self.swarm_config.vision_range;
        let mut in_range = Vec::new();

        for (idx, other) in self.swarms.iter().enumerate() {
            if idx == swarm_idx {
                continue;
            }
            let dist_sq = swarm.center.distance_squared(other.center);
            if dist_sq <= range_sq {
                in_range.push((other, dist_sq.sqrt()));
            }
        }

        in_range.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        in_range
    }

    /// Perform one update of the simulation
    pub fn step(&mut self) {
        // Phase 1: Collect decisions (read-only)
        let decisions: Vec<Option<SwarmDecision>> = (0..self.swarms.len())
            .map(|idx| self.swarms[idx].decide(self, idx))
            .collect();

        // Phase 2: Apply decisions
        for (swarm, decision) in self.swarms.iter_mut().zip(decisions) {
            if let Some(d) = decision {
                swarm.apply_decision(&d);
            }
        }

        // Phase 3: Movement
        for swarm in &mut self.swarms {
            swarm.movement();
        }

        // Phase 4: Combat — each swarm fights nearby enemy ships
        let mut all_hits: Vec<ShipId> = Vec::new();

        for swarm_idx in 0..self.swarms.len() {
            let (before, rest) = self.swarms.split_at_mut(swarm_idx);
            let (swarm, after) = rest.split_first_mut().unwrap();

            let enemies: Vec<&crate::ship::Ship> = before.iter()
                .chain(after.iter())
                .filter(|other| {
                    other.center.distance(swarm.center) <= swarm.config.vision_range
                })
                .flat_map(|s| s.ships.iter().map(|(ship, _)| ship))
                .collect();

            let hits = swarm.fight(&enemies);
            all_hits.extend(hits);
        }

        // apply damage
        for hit_id in &all_hits {
            for swarm in &mut self.swarms {
                for (ship, _) in &mut swarm.ships {
                    if ship.id == *hit_id {
                        ship.health = ship.health.saturating_sub(1);
                    }
                }
            }
        }

        // Phase 5: Finalize
        for swarm in &mut self.swarms {
            swarm.finalize();
        }

        // Phase 5: Cleanup dead swarms
        self.swarms.retain(|s| !s.ships.is_empty());
    }
}
