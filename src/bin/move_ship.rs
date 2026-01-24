use macroquad::prelude::*;

use swarm_simulation::render::draw_ship;
use swarm_simulation::ship::{Ship, ShipConfig};

#[macroquad::main("Move Ship")]
async fn main() {
    let config = ShipConfig::default();
    let start_pos = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
    let mut ship = Ship::spawn(start_pos, &config);

    loop {
        // Handle mouse click to set target
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            ship.set_target(Vec2::new(mx, my));
        }

        // Update ship
        ship.movement();

        // Render
        clear_background(WHITE);
        draw_ship(&ship);

        next_frame().await;
    }
}
