use macroquad::prelude::*;

use swarm_simulation::render::draw_swarm;
use swarm_simulation::simulation::{Bounds, Simulation, SimulationConfig};

const NUM_SWARMS: usize = 5;

fn generate_colors(n: usize) -> Vec<Color> {
    (0..n)
        .map(|_| {
            Color::new(
                rand::gen_range(0.0, 1.0),
                rand::gen_range(0.0, 1.0),
                rand::gen_range(0.0, 1.0),
                1.0,
            )
        })
        .collect()
}

fn random_pos() -> Vec2 {
    Vec2::new(
        rand::gen_range(100.0, screen_width() - 100.0),
        rand::gen_range(100.0, screen_height() - 100.0),
    )
}

#[macroquad::main("Multi Swarm")]
async fn main() {
    let bounds = Bounds::new(screen_width(), screen_height());
    let mut sim = Simulation::new(SimulationConfig::default(), bounds);
    let colors = generate_colors(NUM_SWARMS);

    for _ in 0..NUM_SWARMS {
        let num_ships = rand::gen_range(10, 30);
        sim.spawn_swarm(random_pos(), num_ships);
    }

    loop {
        sim.step();

        // Render
        clear_background(WHITE);
        for (i, swarm) in sim.swarms().iter().enumerate() {
            let color = colors.get(i).copied().unwrap_or(GRAY);
            draw_swarm(swarm, color);
        }

        next_frame().await;
    }
}
