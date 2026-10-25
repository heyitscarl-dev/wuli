use glam::{DVec2, dvec2};
use wuli::{RigidBody, World};

use crate::{ZERO_GRAVITY, wall};

/// A striker hits a row of resting balls; walls send the momentum back and forth
pub fn newtons_row() -> World {
    let r = 0.2;
    let mut world = World::new(ZERO_GRAVITY);
    world.add_body(RigidBody::circle(dvec2(-3.0, 0.0), dvec2(3.0, 0.0), 1.0, r));
    for i in 0..5 {
        let x = i as f64 * (2.0 * r + 0.01); // small gaps keep the collisions sequential
        world.add_body(RigidBody::circle(dvec2(x, 0.0), DVec2::ZERO, 1.0, r));
    }
    world.add_body(wall(dvec2(-4.1, 0.0), dvec2(0.1, 1.0)));
    world.add_body(wall(dvec2(4.1, 0.0), dvec2(0.1, 1.0)));
    world
}

/// A cue ball breaks a rack of 15 balls on a table with elastic cushions
pub fn billiards_break() -> World {
    let r = 0.15;
    let d = 2.0 * r + 0.002;
    let mut world = World::new(ZERO_GRAVITY);

    world.add_body(RigidBody::circle(dvec2(-3.0, 0.0), dvec2(8.0, 0.0), 1.0, r));
    for row in 0..5 {
        for j in 0..=row {
            let x = 1.0 + row as f64 * d * 3f64.sqrt() / 2.0;
            let y = (j as f64 - row as f64 / 2.0) * d;
            world.add_body(RigidBody::circle(dvec2(x, y), DVec2::ZERO, 1.0, r));
        }
    }

    world.add_body(wall(dvec2(0.0, 2.4), dvec2(4.3, 0.1)));
    world.add_body(wall(dvec2(0.0, -2.4), dvec2(4.3, 0.1)));
    world.add_body(wall(dvec2(-4.2, 0.0), dvec2(0.1, 2.5)));
    world.add_body(wall(dvec2(4.2, 0.0), dvec2(0.1, 2.5)));
    world
}

/// A heavy ball pushes a light one against a wall. With a 100:1 mass ratio
/// there are 31 collisions in total, the first digits of π (Galperin, 2003)
pub fn counting_pi() -> World {
    let mut world = World::new(ZERO_GRAVITY);
    world.add_body(RigidBody::circle(dvec2(-2.0, 0.0), DVec2::ZERO, 1.0, 0.1));
    world.add_body(RigidBody::circle(
        dvec2(0.0, 0.0),
        dvec2(-1.0, 0.0),
        0.01,
        0.5,
    ));
    world.add_body(wall(dvec2(-3.1, 0.0), dvec2(0.1, 1.0)));
    world
}
