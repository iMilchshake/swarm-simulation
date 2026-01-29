use macroquad::prelude::*;

use swarm_simulation::camera::MapCamera;
use swarm_simulation::render::draw_swarm;
use swarm_simulation::simulation::{Bounds, Simulation, SimulationConfig};

const NUM_SWARMS: usize = 5;
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
    let mut sim = Simulation::new(SimulationConfig::default(), bounds);
    let colors = generate_colors(NUM_SWARMS);
    let mut camera = MapCamera::new(map_width, map_height);

    for _ in 0..NUM_SWARMS {
        let num_ships = rand::gen_range(10, 30);
        sim.spawn_swarm(random_pos(sim.bounds()), num_ships);
    }

    loop {
        // Camera controls
        let (_scroll_x, scroll_y) = mouse_wheel();
        if scroll_y > 0.0 {
            camera.zoom_in();
        } else if scroll_y < 0.0 {
            camera.zoom_out();
        }

        // Pan with left mouse drag
        let delta = mouse_delta_position();
        if is_mouse_button_down(MouseButton::Left) && !is_mouse_button_pressed(MouseButton::Left) {
            camera.shift(Vec2::new(delta.x, delta.y));
        }

        // Reset camera with R key
        if is_key_pressed(KeyCode::R) {
            camera.reset();
        }

        sim.step();

        // Render
        clear_background(WHITE);
        camera.apply();

        // Draw map bounds
        let bounds = sim.bounds();
        draw_rectangle_lines(bounds.min.x, bounds.min.y, bounds.max.x, bounds.max.y, 2.0, BLACK);

        for (i, swarm) in sim.swarms().iter().enumerate() {
            let color = colors.get(i).copied().unwrap_or(GRAY);
            draw_swarm(swarm, color);
        }

        camera.reset_camera();
        next_frame().await;
    }
}
