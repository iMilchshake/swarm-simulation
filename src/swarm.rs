use std::f32::consts::PI;

use glam::Vec2;

use crate::{
    ship::{Ship, ShipConfig},
    swarm,
};

pub struct SwarmConfig {
    /// maximum number of ships in a swarm
    pub max_ships: u32,

    /// initial number of ships in a swarm
    pub init_ships: u32,

    /// swarm position distribution scale
    pub scale: f32,

    /// maximum vision range to see other swarms
    pub vision_range: f32,
}

impl Default for SwarmConfig {
    fn default() -> Self {
        SwarmConfig {
            init_ships: 30,
            max_ships: 30,
            scale: 10.0,
            vision_range: 200.0,
        }
    }
}

/// Swarm consisting of multiple ships.
/// Ships that are part of the swarm are assigned a position releative to
/// the Swarms target position.
/// The swarms own position is the average position of all ships
pub struct Swarm<'a> {
    /// keeps track of all ships and their **relative** position to swarm
    pub ships: Vec<(Ship<'a>, Vec2)>,

    center_pos: Vec2,

    pub target_pos: Vec2,

    // x(n) = sqrt(n) cos(nφ)
    // y(n) = sqrt(n) sin(nφ)
    config: &'a SwarmConfig,
}

impl<'a> Swarm<'a> {
    /// Spawn a new swarm with n ships at a given location
    pub fn spawn(
        pos: Vec2,
        swarm_config: &'a SwarmConfig,
        ship_config: &'a ShipConfig,
    ) -> Swarm<'a> {
        let mut ships = Vec::new();
        let golden_ratio = PI * (3.0 - f32::sqrt(5.0));

        for ship_idx in 0..swarm_config.init_ships {
            let n = ship_idx as f32;
            let x = f32::sqrt(n) * f32::cos(n * golden_ratio) * swarm_config.scale;
            let y = f32::sqrt(n) * f32::sin(n * golden_ratio) * swarm_config.scale;
            let ship = Ship::spawn(Vec2::new(x, y), ship_config);

            ships.push((ship, Vec2::new(x, y)));
        }

        Swarm {
            ships: ships,
            center_pos: pos,
            target_pos: pos,
            config: swarm_config,
        }
    }

    pub fn set_target(&mut self, pos: Vec2) {
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

        // TODO: update swarm position
    }
}
