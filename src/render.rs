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

    // shot fired this tick: thick bright line
    if let Some(target_pos) = ship.fired_at {
        draw_line(
            pos.x,
            pos.y,
            target_pos.x,
            target_pos.y,
            2.0,
            color.with_alpha(1.0),
        );
    }
    // lock-on in progress: thin faint line
    else if let Some(target_pos) = ship.lock_target_pos {
        draw_line(
            pos.x,
            pos.y,
            target_pos.x,
            target_pos.y,
            1.,
            color.with_alpha(0.33),
        );
    }
}

pub fn draw_swarm(swarm: &Swarm, color: Color) {
    for (ship, _) in &swarm.ships {
        draw_ship(ship, color);
    }

    draw_circle(
        swarm.target_pos.x,
        swarm.target_pos.y,
        2.0,
        color.with_alpha(0.50),
    );

    draw_circle_lines(
        swarm.center.x,
        swarm.center.y,
        swarm.config.vision_range,
        1.0,
        color.with_alpha(0.25),
    );

    draw_circle(swarm.center.x, swarm.center.y, 4.0, color.with_alpha(0.25));

    draw_line(
        swarm.center.x,
        swarm.center.y,
        swarm.target_pos.x,
        swarm.target_pos.y,
        1.0,
        color.with_alpha(0.25),
    );
}

pub fn draw_background_cover(texture: &Texture2D, aspect_ratio: f32) {
    let screen_aspect = screen_width() / screen_height();

    let (render_w, render_h) = if screen_aspect > aspect_ratio {
        (screen_width(), screen_width() / aspect_ratio)
    } else {
        (screen_height() * aspect_ratio, screen_height())
    };

    draw_texture_ex(
        texture,
        0.0,
        0.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(render_w, render_h)),
            ..Default::default()
        },
    );
}
