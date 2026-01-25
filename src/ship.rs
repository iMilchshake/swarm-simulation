use glam::Vec2;
use std::rc::Rc;

const EPSILON: f32 = 0.001;

#[derive(Clone)]
pub struct ShipConfig {
    /// maximum ship velocity magnitude
    pub max_speed: f32,
    /// maximum acceleration (thrust)
    pub max_accel: f32,
    /// maximum deceleration (braking)
    pub max_decel: f32,
    /// max range to lock onto enemy ship
    pub aim_range: f32,
    /// delay after a shot in ticks
    pub fire_delay: u32,
    /// initial ship health points (1 hit = 1 damage)
    pub health: u32,
}

impl Default for ShipConfig {
    fn default() -> Self {
        ShipConfig {
            max_speed: 10.0,
            max_accel: 0.15,
            max_decel: 0.1,
            aim_range: 50.0,
            fire_delay: 10,
            health: 2,
        }
    }
}

/// A single unit. Controlled by a swarm, but works independent.
pub struct Ship {
    pub pos: Vec2,
    pub vel: Vec2,
    pub target_pos: Vec2,
    pub health: u32,
    config: Rc<ShipConfig>,
}

impl Ship {
    pub fn spawn(pos: Vec2, config: Rc<ShipConfig>) -> Ship {
        Ship {
            pos,
            vel: Vec2::ZERO,
            target_pos: pos,
            health: config.health,
            config,
        }
    }

    pub fn set_target(&mut self, pos: Vec2) {
        self.target_pos = pos;
    }

    pub fn fight(&mut self) {
        // todo!();
    }

    /// TODO: right now ships just accelerate in the direction of target,
    /// or break by decelerating in the opposite direction of velocity
    /// this is not optimal and could lead to ships not reaching their
    /// destination optimally -> gotta investiage :)
    /// A smarter solution would allow to determine a smart acceleration
    /// vector that leads to the optimal / shortest path to the target
    pub fn movement(&mut self) {
        let to_target = self.target_pos - self.pos;
        let dist = to_target.length();
        let speed = self.vel.length();

        // close and slow enough -> full stop
        if dist < EPSILON && speed < EPSILON {
            self.vel = Vec2::ZERO;
            self.pos = self.target_pos;
            return;
        }

        if dist < EPSILON {
            // at target but still moving -> brake
            let brake = speed.min(self.config.max_decel);
            self.vel = self.vel.normalize() * (speed - brake);
            self.pos += self.vel;
            return;
        }

        let dir_to_target = to_target / dist;

        // Max safe approach speed
        let decel = self.config.max_decel;
        let v_max = -decel + (decel * decel + 2.0 * decel * dist).sqrt();
        let v_max = v_max.min(self.config.max_speed);

        // Desired velocity: toward target at v_max speed
        let desired_vel = dir_to_target * v_max;

        // Steer toward desired velocity
        let delta = desired_vel - self.vel;
        let delta_mag = delta.length();

        if delta_mag > EPSILON {
            // Use max_accel for steering/speeding up, max_decel for slowing down
            let max_change = if self.vel.dot(delta) < 0.0 {
                // delta is mostly opposing current velocity -> braking
                self.config.max_decel
            } else {
                self.config.max_accel
            };

            let accel = if delta_mag > max_change {
                delta / delta_mag * max_change
            } else {
                delta
            };

            self.vel += accel;
        }

        // Clamp to max speed
        if self.vel.length() > self.config.max_speed {
            self.vel = self.vel.normalize() * self.config.max_speed;
        }

        self.pos += self.vel;
    }
}
