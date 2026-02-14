use std::sync::atomic::{AtomicU64, Ordering};

use glam::Vec2;
use std::rc::Rc;

const EPSILON: f32 = 0.001;

static NEXT_SHIP_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShipId(pub u64);

impl ShipId {
    pub fn next() -> Self {
        ShipId(NEXT_SHIP_ID.fetch_add(1, Ordering::Relaxed))
    }
}

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
    /// lock time multiplier on fire_delay, scaled by target speed (1.0 = no extra, 5.0 = up to 5x at max speed)
    pub lock_time_factor: f32,
    /// initial ship health points (1 hit = 1 damage)
    pub health: u32,
}

impl Default for ShipConfig {
    fn default() -> Self {
        ShipConfig {
            max_speed: 10.0,
            max_accel: 0.30,
            max_decel: 0.30,
            aim_range: 250.0,
            fire_delay: 60,
            lock_time_factor: 2.0,
            health: 3,
        }
    }
}

/// A single unit. Controlled by a swarm, but works independent.
pub struct Ship {
    pub id: ShipId,
    pub pos: Vec2,
    pub vel: Vec2,
    pub target_pos: Vec2,
    pub health: u32,
    pub config: Rc<ShipConfig>,

    /// current lock-on target
    pub lock_target: Option<ShipId>,
    /// ticks spent locking onto current target
    pub lock_progress: u32,
    /// set to target position on the tick a shot fires (for rendering)
    pub fired_at: Option<Vec2>,
    /// position of current lock target (for rendering lock-on line)
    pub lock_target_pos: Option<Vec2>,
}

impl Ship {
    pub fn spawn(pos: Vec2, config: Rc<ShipConfig>) -> Ship {
        Ship {
            id: ShipId::next(),
            pos,
            vel: Vec2::ZERO,
            target_pos: pos,
            health: config.health,
            config,
            lock_target: None,
            lock_progress: 0,
            fired_at: None,
            lock_target_pos: None,
        }
    }

    pub fn speed(&self) -> f32 {
        self.vel.length()
    }

    pub fn set_target(&mut self, pos: Vec2) {
        self.target_pos = pos;
    }

    /// TODO: right now ships just accelerate in the direction of target,
    /// or break by decelerating in the opposite direction of velocity
    /// this is not optimal and could lead to ships not reaching their
    /// destination optimally -> gotta investiage :)
    /// A smarter solution would allow to determine a smart acceleration
    /// vector that leads to the optimal / shortest path to the target
    pub fn movement(&mut self, accel_factor: f32) {
        let to_target = self.target_pos - self.pos;
        let dist = to_target.length();
        let speed = self.vel.length();
        let max_accel = self.config.max_accel * accel_factor;
        let max_decel = self.config.max_decel * accel_factor;

        // close and slow enough -> full stop
        if dist < EPSILON && speed < EPSILON {
            self.vel = Vec2::ZERO;
            self.pos = self.target_pos;
            return;
        }

        if dist < EPSILON {
            // at target but still moving -> brake
            let brake = speed.min(max_decel);
            self.vel = self.vel.normalize() * (speed - brake);
            self.pos += self.vel;
            return;
        }

        let dir_to_target = to_target / dist;

        // Max safe approach speed
        let v_max = -max_decel + (max_decel * max_decel + 2.0 * max_decel * dist).sqrt();
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
                max_decel
            } else {
                max_accel
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
