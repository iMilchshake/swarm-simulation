use crate::ship::ShipConfig;
use crate::swarm::{Swarm, SwarmConfig};

pub struct SimulationConfig {
    pub ship: ShipConfig,
    pub swarm: SwarmConfig,

    /// maximum number of swarms in the simulation
    pub max_swarms: u32,

    // initial number of swarms
    pub init_swarms: u32,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        SimulationConfig {
            ship: ShipConfig::default(),
            swarm: SwarmConfig::default(),
            max_swarms: 10,
            init_swarms: 2,
        }
    }
}

pub struct Simulation<'a> {
    pub swarms: Vec<Swarm<'a>>,
    config: SimulationConfig,
}

impl<'a> Simulation<'a> {
    pub fn new(config: SimulationConfig) -> Simulation<'a> {
        Simulation {
            swarms: vec![],
            config,
        }
    }

    /// perform one update of the simulation
    pub fn step(&mut self) {
        for swarm in &mut self.swarms {
            swarm.fight();
        }

        for swarm in &mut self.swarms {
            swarm.movement();
        }

        for swarm in &mut self.swarms {
            swarm.finalize();
        }
    }
}
