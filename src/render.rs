use macroquad::prelude::*;

use crate::ship::Ship;

pub fn draw_ship(ship: &Ship) {
    let pos = ship.pos;
    let vel = ship.vel;

    // Draw ship as a triangle pointing in velocity direction
    let size = 10.0;
    let angle = if vel.length() > 0.001 {
        vel.y.atan2(vel.x)
    } else {
        0.0
    };

    let front = Vec2::new(angle.cos(), angle.sin()) * size;
    let back_left = Vec2::new((angle + 2.4).cos(), (angle + 2.4).sin()) * size * 0.6;
    let back_right = Vec2::new((angle - 2.4).cos(), (angle - 2.4).sin()) * size * 0.6;

    draw_triangle(
        vec2(pos.x + front.x, pos.y + front.y),
        vec2(pos.x + back_left.x, pos.y + back_left.y),
        vec2(pos.x + back_right.x, pos.y + back_right.y),
        BLUE,
    );

    // Draw target position
    draw_circle(ship.target_pos.x, ship.target_pos.y, 5.0, RED);
}
