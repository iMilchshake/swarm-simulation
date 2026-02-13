use macroquad::miniquad::conf::Platform;
use macroquad::prelude::*;

use macroquad_viewplane_camera::ViewplaneCamera;

use swarm_simulation::render::{draw_background_cover, draw_swarm};
use swarm_simulation::simulation::{Bounds, Simulation, SimulationConfig};

const NUM_SWARMS: usize = 25;
const MAP_WIDTH: f32 = 1980.;
const MAP_HEIGHT: f32 = 1980.;
const SIM_FRAME_TIME: f64 = 1. / 60.;

fn window_conf() -> Conf {
    Conf {
        window_title: "Multi Swarm".to_owned(),
        platform: Platform {
            swap_interval: Some(1), // -1 = adaptive vsync
            ..Default::default()
        },
        ..Default::default()
    }
}

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

#[macroquad::main(window_conf)]
async fn main() {
    let bounds = Bounds::new(MAP_WIDTH, MAP_HEIGHT);
    let mut camera = ViewplaneCamera::new(MAP_WIDTH, MAP_HEIGHT);

    let mut sim = Simulation::new(SimulationConfig::default(), bounds);

    let colors = generate_colors(NUM_SWARMS);

    let background = load_texture("assets/backgrounds/space_background1.png")
        .await
        .unwrap();

    for _ in 0..NUM_SWARMS {
        let num_ships = rand::gen_range(2, 30);
        sim.spawn_swarm(random_pos(sim.bounds()), num_ships);
    }

    let mut sim_time_lag = 0.0;

    loop {
        camera.set_viewport(0, 0, screen_width() as i32, screen_height() as i32);
        camera.handle_inputs();

        // run simulation steps needed to catch up, but dont exceed target simulation speed
        sim_time_lag += get_frame_time() as f64;
        while sim_time_lag >= SIM_FRAME_TIME {
            sim.step();
            sim_time_lag -= SIM_FRAME_TIME;
        }

        clear_background(WHITE);

        draw_background_cover(&background, 16. / 9.);

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
