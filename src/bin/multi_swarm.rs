use macroquad::prelude::*;

use swarm_simulation::render::draw_swarm;
use swarm_simulation::ship::ShipConfig;
use swarm_simulation::swarm::{Swarm, SwarmConfig};

const NUM_SWARMS: usize = 5;

fn generate_colors() -> [Color; NUM_SWARMS] {
    std::array::from_fn(|_| {
        Color::new(
            rand::gen_range(0.0, 1.0),
            rand::gen_range(0.0, 1.0),
            rand::gen_range(0.0, 1.0),
            1.0,
        )
    })
}

fn random_pos() -> Vec2 {
    Vec2::new(
        rand::gen_range(100.0, screen_width() - 100.0),
        rand::gen_range(100.0, screen_height() - 100.0),
    )
}

struct SwarmState<'a> {
    swarm: Swarm<'a>,
    start_pos: Vec2,
}

impl<'a> SwarmState<'a> {
    fn progress(&self) -> f32 {
        let total_dist = (self.swarm.target_pos - self.start_pos).length();
        if total_dist < 1.0 {
            return 1.0;
        }
        let remaining_dist = (self.swarm.target_pos - self.swarm.center).length();
        1.0 - (remaining_dist / total_dist).clamp(0.0, 1.0)
    }

    fn set_new_target(&mut self) {
        self.start_pos = self.swarm.center;
        self.swarm.set_target(random_pos());
    }
}

#[macroquad::main("Multi Swarm")]
async fn main() {
    let ship_config = ShipConfig::default();
    let swarm_config = SwarmConfig::default();
    let swarm_colors = generate_colors();

    let mut swarms: Vec<SwarmState> = Vec::new();

    for _ in 0..NUM_SWARMS {
        let pos = random_pos();
        let mut swarm = Swarm::spawn(pos, &swarm_config, &ship_config);
        swarm.set_target(random_pos());
        swarms.push(SwarmState {
            start_pos: pos,
            swarm,
        });
    }

    let mut tunnel_chaser = Swarm::spawn(random_pos(), &swarm_config, &ship_config);
    let mut nearest_chaser = Swarm::spawn(random_pos(), &swarm_config, &ship_config);

    loop {
        // Update swarms and check progress
        for state in &mut swarms {
            state.swarm.movement();
            state.swarm.finalize();

            if state.progress() >= 0.50 {
                state.set_new_target();
            }
        }

        tunnel_chaser.set_target(swarms[0].swarm.center);
        tunnel_chaser.movement();
        tunnel_chaser.finalize();

        let mut min_dist = f32::MAX;
        let mut nearest_target = nearest_chaser.center;
        for state in &mut swarms {
            let dist = state.swarm.center.distance_squared(nearest_chaser.center);
            if dist < min_dist {
                nearest_target = state.swarm.center;
                min_dist = dist;
            }
        }
        nearest_chaser.set_target(nearest_target);
        nearest_chaser.movement();
        nearest_chaser.finalize();

        // Render
        clear_background(WHITE);
        for (i, state) in swarms.iter().enumerate() {
            draw_swarm(&state.swarm, swarm_colors[i]);
        }

        draw_swarm(&tunnel_chaser, RED);
        draw_swarm(&nearest_chaser, RED);

        next_frame().await;
    }
}
