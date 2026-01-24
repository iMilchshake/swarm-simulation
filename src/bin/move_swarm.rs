use macroquad::prelude::*;

use swarm_simulation::render::draw_swarm;
use swarm_simulation::ship::ShipConfig;
use swarm_simulation::swarm::{Swarm, SwarmConfig};

#[macroquad::main("Move Swarm")]
async fn main() {
    let ship_config = ShipConfig::default();
    let swarm_config = SwarmConfig::default();

    let start_pos = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
    let mut swarm = Swarm::spawn(start_pos, &swarm_config, &ship_config);

    loop {
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            swarm.set_target(Vec2::new(mx, my));
        }

        swarm.movement();

        clear_background(WHITE);
        draw_swarm(&swarm, BLUE);

        next_frame().await;
    }
}
