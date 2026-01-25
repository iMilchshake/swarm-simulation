use macroquad::prelude::*;

use crate::ship::Ship;
use crate::swarm::Swarm;

pub fn draw_ship(ship: &Ship, color: Color) {
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
        color,
    );
}

pub fn draw_swarm(swarm: &Swarm, color: Color) {
    for (ship, _) in &swarm.ships {
        draw_ship(ship, color);
    }

    draw_circle(
        swarm.target_pos.x,
        swarm.target_pos.y,
        3.0,
        color.with_alpha(0.5),
    );

    draw_circle_lines(
        swarm.center.x,
        swarm.center.y,
        swarm.config.vision_range,
        0.5,
        color.with_alpha(0.5),
    );

    draw_circle(swarm.center.x, swarm.center.y, 8.0, color.with_alpha(1.0));

    draw_line(
        swarm.center.x,
        swarm.center.y,
        swarm.target_pos.x,
        swarm.target_pos.y,
        1.0,
        color.with_alpha(0.5),
    );
}
