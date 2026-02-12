use macroquad::prelude::*;

use macroquad_viewplane_camera::ViewplaneCamera;

use swarm_simulation::render::draw_swarm;
use swarm_simulation::simulation::{Bounds, Simulation, SimulationConfig};

const NUM_SWARMS: usize = 25;
const MAP_SCALE: f32 = 2.0;

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

fn random_pos(bounds: &Bounds) -> Vec2 {
    Vec2::new(
        rand::gen_range(100.0, bounds.max.x - 100.0),
        rand::gen_range(100.0, bounds.max.y - 100.0),
    )
}

#[macroquad::main("Multi Swarm")]
async fn main() {
    let map_width = screen_width() * MAP_SCALE;
    let map_height = screen_height() * MAP_SCALE;
    let bounds = Bounds::new(map_width, map_height);
    let mut camera = ViewplaneCamera::new(map_width, map_height);

    let mut sim = Simulation::new(SimulationConfig::default(), bounds);
    let colors = generate_colors(NUM_SWARMS);

    for _ in 0..NUM_SWARMS {
        let num_ships = rand::gen_range(2, 30);
        sim.spawn_swarm(random_pos(sim.bounds()), num_ships);
    }

    loop {
        camera.set_viewport(0, 0, screen_width() as i32, screen_height() as i32);
        camera.handle_inputs();

        sim.step();

        clear_background(WHITE);
        camera.apply();

        let bounds = sim.bounds();
        draw_rectangle_lines(
            bounds.min.x,
            bounds.min.y,
            bounds.max.x,
            bounds.max.y,
            4.0,
            BLACK,
        );

        for (i, swarm) in sim.swarms().iter().enumerate() {
            let color = colors.get(i).copied().unwrap_or(GRAY);
            draw_swarm(swarm, color);
        }

        camera.reset_camera();
        next_frame().await;
    }
}
