use glam::Vec2;
use std::rc::Rc;

use crate::ship::ShipConfig;
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

    pub fn nearest_bound_edge(&self, pos: Vec2) -> Vec2 {
        let left = pos.x - self.min.x;
        let top = pos.y - self.min.y;
        let right = self.max.x - pos.x;
        let bot = self.max.y - pos.y;

        let min_dist = left.min(top).min(right).min(bot);

        if left == min_dist {
            Vec2::new(0.0, pos.y)
        } else if top == min_dist {
            Vec2::new(pos.x, 0.0)
        } else if right == min_dist {
            Vec2::new(self.max.x, pos.y)
        } else if bot == min_dist {
            Vec2::new(pos.x, self.max.y)
        } else {
            unreachable!()
        }
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

        dbg!(&decisions);

        // Phase 2: Apply decisions
        for (swarm, decision) in self.swarms.iter_mut().zip(decisions) {
            if let Some(d) = decision {
                swarm.apply_decision(&d);
            }
        }

        // Phase 3: Simulate Physics
        for swarm in &mut self.swarms {
            swarm.movement();
        }
        for swarm in &mut self.swarms {
            swarm.fight();
        }
        for swarm in &mut self.swarms {
            swarm.finalize();
        }

        // Phase 5: Cleanup dead swarms
        self.swarms.retain(|s| !s.ships.is_empty());
    }
}
