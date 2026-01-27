use glam::Vec2;
use std::f32::consts::TAU;

use crate::simulation::Bounds;

const NUM_ANGLES: usize = 64;
const ANGLE_STEP: f32 = TAU / NUM_ANGLES as f32;

/// Gaussian function for smooth weight falloff
fn gaussian(x: f32, sigma: f32) -> f32 {
    (-x * x / (2.0 * sigma * sigma)).exp()
}

/// Computes the shortest angular distance between two angles (in radians)
fn angle_diff(a: f32, b: f32) -> f32 {
    let diff = (a - b).rem_euclid(TAU);
    if diff > std::f32::consts::PI {
        diff - TAU
    } else {
        diff
    }
}

/// Converts an angle bucket index to its corresponding angle in radians
fn bucket_to_angle(bucket: usize) -> f32 {
    bucket as f32 * ANGLE_STEP
}

/// A map of directional weights used to compute the best flee direction.
/// Each bucket represents a discrete angle. Repulsors subtract from weights,
/// and the angle with the highest remaining weight is chosen as the flee direction.
pub struct RepulsionMap {
    weights: [f32; NUM_ANGLES],
}

impl RepulsionMap {
    /// Create a new repulsion map with all weights initialized to 0
    pub fn new() -> Self {
        RepulsionMap {
            weights: [0.0; NUM_ANGLES],
        }
    }

    /// Add a repulsor at the given angle with gaussian decay to neighboring buckets.
    /// The strength is subtracted from the weight at the center angle and decays
    /// to neighboring angles based on sigma.
    pub fn add_repulsor(&mut self, angle: f32, strength: f32, sigma: f32) {
        for bucket in 0..NUM_ANGLES {
            let bucket_angle = bucket_to_angle(bucket);
            let diff = angle_diff(bucket_angle, angle).abs();
            let weight = strength * gaussian(diff, sigma);
            self.weights[bucket] -= weight;
        }
    }

    /// Add wall repulsion by raycasting from the given position in each direction.
    /// Walls closer than detect_range will add repulsion proportional to proximity.
    /// The total wall repulsion is normalized to prevent walls from overwhelming
    /// enemy repulsors.
    pub fn add_wall_repulsion(
        &mut self,
        pos: Vec2,
        bounds: &Bounds,
        detect_range: f32,
        sigma: f32,
    ) {
        const MAX_WALL_WEIGHT: f32 = 1.5;

        let mut wall_repulsors: Vec<(f32, f32)> = Vec::new(); // (angle, strength)
        let mut total_strength = 0.0;

        for bucket in 0..NUM_ANGLES {
            let angle = bucket_to_angle(bucket);
            let dir = Vec2::from_angle(angle);

            // Raycast to find distance to wall in this direction
            let dist = raycast_to_bounds(pos, dir, bounds);

            if dist < detect_range {
                // Closer = stronger repulsion (1.0 at wall, 0.0 at range limit)
                let strength = 1.0 - (dist / detect_range);
                wall_repulsors.push((angle, strength));
                total_strength += strength;
            }
        }

        // Normalize wall repulsion to prevent overwhelming enemy repulsors
        let scale = if total_strength > MAX_WALL_WEIGHT {
            MAX_WALL_WEIGHT / total_strength
        } else {
            1.0
        };

        for (angle, strength) in wall_repulsors {
            self.add_repulsor(angle, strength * scale, sigma);
        }
    }

    /// Penalize angles that require sharp turns from the current velocity direction.
    /// This encourages smoother, more momentum-preserving movement.
    pub fn add_velocity_penalty(&mut self, velocity: Vec2, strength: f32, sigma: f32) {
        if velocity.length_squared() < 0.001 {
            return; // No velocity, no penalty
        }

        let current_heading = velocity.to_angle();
        // The opposite direction to current heading should be most penalized
        let opposite = (current_heading + std::f32::consts::PI).rem_euclid(TAU);

        self.add_repulsor(opposite, strength, sigma);
    }

    /// Return the angle with the highest weight (least repulsion)
    pub fn best_angle(&self) -> f32 {
        let mut best_bucket = 0;
        let mut best_weight = self.weights[0];

        for bucket in 1..NUM_ANGLES {
            if self.weights[bucket] > best_weight {
                best_weight = self.weights[bucket];
                best_bucket = bucket;
            }
        }

        bucket_to_angle(best_bucket)
    }
}

impl Default for RepulsionMap {
    fn default() -> Self {
        Self::new()
    }
}

/// Raycast from a position in a given direction to find the distance to the boundary.
/// Returns the distance to the closest wall intersection.
fn raycast_to_bounds(pos: Vec2, dir: Vec2, bounds: &Bounds) -> f32 {
    let mut min_dist = f32::MAX;

    // Check each wall
    // Left wall (x = min.x)
    if dir.x < 0.0 {
        let t = (bounds.min.x - pos.x) / dir.x;
        if t > 0.0 {
            min_dist = min_dist.min(t);
        }
    }

    // Right wall (x = max.x)
    if dir.x > 0.0 {
        let t = (bounds.max.x - pos.x) / dir.x;
        if t > 0.0 {
            min_dist = min_dist.min(t);
        }
    }

    // Top wall (y = min.y)
    if dir.y < 0.0 {
        let t = (bounds.min.y - pos.y) / dir.y;
        if t > 0.0 {
            min_dist = min_dist.min(t);
        }
    }

    // Bottom wall (y = max.y)
    if dir.y > 0.0 {
        let t = (bounds.max.y - pos.y) / dir.y;
        if t > 0.0 {
            min_dist = min_dist.min(t);
        }
    }

    min_dist
}
