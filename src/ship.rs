use glam::Vec2;

const EPSILON: f32 = 0.001;

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
            max_speed: 5.0,
            max_accel: 0.5,
            max_decel: 2.0,
            aim_range: 50.0,
            fire_delay: 10,
            health: 2,
        }
    }
}

/// A single unit. Controlled by a swarm, but works independent.
pub struct Ship<'a> {
    pub pos: Vec2,
    pub vel: Vec2,
    pub target_pos: Vec2,
    pub health: u32,
    config: &'a ShipConfig,
}

impl<'a> Ship<'a> {
    pub fn spawn(pos: Vec2, config: &'a ShipConfig) -> Ship<'a> {
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
        todo!();
    }

    /// TODO: right now ships just accelerate in the direction of target,
    /// or break by decelerating in the opposite direction of velocity
    /// this is not optimal and could lead to ships not reaching their
    /// destination -> gotta investiage :)
    /// A smarter solution would allow to determine a smart acceleration
    /// vector that leads to the optimal / shortest path to the target
    pub fn movement(&mut self) {
        let to_target = self.target_pos - self.pos;
        let dist = to_target.length();
        let speed = self.vel.length();

        // close enough and slow enough -> stop
        if dist < 0.1 && speed < 0.1 {
            self.vel = Vec2::ZERO;
            self.pos += self.vel;
            return;
        }

        if dist > EPSILON {
            // max speed we can have and still stop in time``
            // kinetic equation v^2 = 2*a*d => v = sqrt(2*a*d)
            let max_safe_speed = (2.0 * dist * self.config.max_decel).sqrt();
            let max_safe_speed = max_safe_speed.min(self.config.max_speed);

            if speed > max_safe_speed {
                // too fast -> brake opposite to velocity
                let brake_dir = -self.vel.normalize();
                let decel = (brake_dir * self.config.max_decel).clamp_length_max(speed);
                self.vel += decel;
            } else if speed + self.config.max_accel < max_safe_speed {
                // can accelerate toward target without exceeding safe speed
                let desired_dir = to_target.normalize();
                self.vel += desired_dir * self.config.max_accel;

                self.vel = self.vel.clamp_length_max(self.config.max_speed);
            }
        }

        self.pos += self.vel;
    }
}
