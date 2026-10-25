use glam::{DVec2, dvec2};
use wuli::{RigidBody, World};

use crate::{NORMAL_GRAVITY, wall};

/// Balls fall through a triangular grid of pegs into bins
pub fn galton_board() -> World {
    let mut world = World::new(NORMAL_GRAVITY);

    for row in 0..7 {
        let y = 2.4 - row as f64 * 0.45;
        let offset = if row % 2 == 0 { 0.0 } else { 0.25 };
        for col in -6..=6 {
            let x = col as f64 * 0.5 + offset;
            if x.abs() < 3.0 {
                world.add_body(RigidBody::circle(dvec2(x, y), DVec2::ZERO, 0.0, 0.05));
            }
        }
    }

    for i in -6..=6 {
        world.add_body(wall(dvec2(i as f64 * 0.5, -2.9), dvec2(0.02, 0.5)));
    }
    world.add_body(wall(dvec2(0.0, -3.5), dvec2(3.3, 0.1)));
    world.add_body(wall(dvec2(-3.2, 0.0), dvec2(0.1, 3.6)));
    world.add_body(wall(dvec2(3.2, 0.0), dvec2(0.1, 3.6)));
    // Collisions are perfectly elastic, so without a lid some balls bounce over the walls
    world.add_body(wall(dvec2(0.0, 3.7), dvec2(3.3, 0.1)));

    // Three rows of four, each row shifted a little, so the balls don't all follow the same path
    for i in 0..12 {
        let (row, col) = (i / 4, i % 4);
        let x = (col as f64 - 1.5) * 0.17 + row as f64 * 0.03;
        let y = 3.0 + row as f64 * 0.2;
        world.add_body(RigidBody::circle(dvec2(x, y), DVec2::ZERO, 1.0, 0.07));
    }

    world
}
