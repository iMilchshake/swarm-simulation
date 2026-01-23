use glam::Vec2;

use crate::ship::Ship;

pub struct SwarmConfig {
    /// maximum number of ships in a swarm
    pub max_ships: u32,
    /// maximum vision range to see other swarms
    pub vision_range: f32,
}

impl Default for SwarmConfig {
    fn default() -> Self {
        SwarmConfig {
            max_ships: 20,
            vision_range: 200.0,
        }
    }
}

/// Swarm consisting of multiple ships.
/// Ships that are part of the swarm are assigned a position releative to
/// the Swarms target position.
/// The swarms own position is the average position of all ships
pub struct Swarm<'a> {
    ships: Vec<Ship<'a>>,

    pos: Vec2,
    target_pos: Vec2,
    target_dir: f32,

    config: &'a SwarmConfig,
}

impl<'a> Swarm<'a> {
    /// Spawn a new swarm with n ships at a given location
    pub fn spawn(pos: Vec2, config: &'a SwarmConfig) -> Swarm<'a> {
        Swarm {
            ships: vec![], // TODO: spawn ships
            pos,
            target_pos: pos,
            target_dir: 0.0, // TODO: start with random dir?
            config,
        }
    }

    pub fn fight(&mut self) {
        for ship in &mut self.ships {
            ship.fight();
        }
    }

    pub fn movement(&mut self) {
        for ship in &mut self.ships {
            ship.movement();
        }
    }

    pub fn finalize(&mut self) {
        self.ships.retain(|ship| ship.health > 0);
    }
}
