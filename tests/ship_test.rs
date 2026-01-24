use glam::Vec2;
use swarm_simulation::ship::{Ship, ShipConfig};

fn test_ship_reaches_target(config: ShipConfig, target_pos: Vec2) {
    let mut ship = Ship::spawn(Vec2::ZERO, &config);
    ship.set_target(target_pos);

    let dist = target_pos.length();
    let max_ticks = 100;

    let mut last_dist = dist;
    for tick in 0..max_ticks {
        ship.movement();

        let dist_to_target = (ship.pos - ship.target_pos).length();
        eprintln!(
            "tick={}, dist={}, pos={}, vel={}",
            tick, dist_to_target, ship.pos, ship.vel
        );
        assert!(
            dist_to_target <= last_dist,
            "overshot at tick {}: dist {} > last {}",
            tick,
            dist_to_target,
            last_dist
        );
        last_dist = dist_to_target;
    }

    let dist_to_target = (ship.pos - ship.target_pos).length();
    assert!(
        dist_to_target < 0.001,
        "failed to reach target: dist={}",
        dist_to_target
    );
    assert!(
        ship.vel.length() < 0.001,
        "failed to stop: vel={}",
        ship.vel.length()
    );
    // println!("SUCCESS -> {}", dist_to_target);
}

#[test]
fn ship_reaches_target_1d_balanced() {
    test_ship_reaches_target(
        ShipConfig {
            max_speed: 10.0,
            max_accel: 1.0,
            max_decel: 1.0,
            ..Default::default()
        },
        Vec2::new(50.0, 0.0),
    );
}

#[test]
fn ship_reaches_target_1d_high_accel() {
    test_ship_reaches_target(
        ShipConfig {
            max_speed: 10.0,
            max_accel: 5.0,
            max_decel: 1.0,
            ..Default::default()
        },
        Vec2::new(50.0, 0.0),
    );
}

#[test]
fn ship_reaches_target_1d_high_decel() {
    test_ship_reaches_target(
        ShipConfig {
            max_speed: 10.0,
            max_accel: 1.0,
            max_decel: 5.0,
            ..Default::default()
        },
        Vec2::new(50.0, 0.0),
    );
}

#[test]
fn ship_reaches_target_1d_short_distance() {
    test_ship_reaches_target(
        ShipConfig {
            max_speed: 10.0,
            max_accel: 1.0,
            max_decel: 1.0,
            ..Default::default()
        },
        Vec2::new(5.0, 0.0),
    );
}

#[test]
fn ship_reaches_target_1d_long_distance() {
    test_ship_reaches_target(
        ShipConfig {
            max_speed: 10.0,
            max_accel: 1.0,
            max_decel: 1.0,
            ..Default::default()
        },
        Vec2::new(500.0, 0.0),
    );
}

#[test]
fn ship_reaches_target_1d_low_accel() {
    test_ship_reaches_target(
        ShipConfig {
            max_speed: 10.0,
            max_accel: 0.1,
            max_decel: 0.1,
            ..Default::default()
        },
        Vec2::new(50.0, 0.0),
    );
}

#[test]
fn ship_reaches_target_2d_balanced() {
    test_ship_reaches_target(
        ShipConfig {
            max_speed: 10.0,
            max_accel: 1.0,
            max_decel: 1.0,
            ..Default::default()
        },
        Vec2::new(30.0, 40.0),
    );
}

#[test]
fn ship_reaches_target_2d_high_accel() {
    test_ship_reaches_target(
        ShipConfig {
            max_speed: 10.0,
            max_accel: 5.0,
            max_decel: 1.0,
            ..Default::default()
        },
        Vec2::new(30.0, 40.0),
    );
}

#[test]
fn ship_reaches_target_2d_high_decel() {
    test_ship_reaches_target(
        ShipConfig {
            max_speed: 10.0,
            max_accel: 1.0,
            max_decel: 5.0,
            ..Default::default()
        },
        Vec2::new(30.0, 40.0),
    );
}

#[test]
fn ship_reaches_target_2d_short_distance() {
    test_ship_reaches_target(
        ShipConfig {
            max_speed: 10.0,
            max_accel: 1.0,
            max_decel: 1.0,
            ..Default::default()
        },
        Vec2::new(3.0, 4.0),
    );
}

#[test]
fn ship_reaches_target_2d_long_distance() {
    test_ship_reaches_target(
        ShipConfig {
            max_speed: 10.0,
            max_accel: 1.0,
            max_decel: 1.0,
            ..Default::default()
        },
        Vec2::new(300.0, 400.0),
    );
}

#[test]
fn ship_reaches_target_2d_low_accel() {
    test_ship_reaches_target(
        ShipConfig {
            max_speed: 10.0,
            max_accel: 0.1,
            max_decel: 0.1,
            ..Default::default()
        },
        Vec2::new(30.0, 40.0),
    );
}

#[test]
fn ship_reaches_target_2d_diagonal() {
    test_ship_reaches_target(
        ShipConfig {
            max_speed: 10.0,
            max_accel: 1.0,
            max_decel: 1.0,
            ..Default::default()
        },
        Vec2::new(50.0, 50.0),
    );
}

#[test]
fn ship_reaches_target_1d_high_speed() {
    test_ship_reaches_target(
        ShipConfig {
            max_speed: 1000.0,
            max_accel: 10.0,
            max_decel: 10.0,
            ..Default::default()
        },
        Vec2::new(50.0, 0.0),
    );
}

#[test]
fn ship_reaches_target_2d_high_speed() {
    test_ship_reaches_target(
        ShipConfig {
            max_speed: 1000.0,
            max_accel: 10.0,
            max_decel: 10.0,
            ..Default::default()
        },
        Vec2::new(30.0, 40.0),
    );
}
