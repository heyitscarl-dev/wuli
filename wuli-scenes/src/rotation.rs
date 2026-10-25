use glam::{DVec2, dvec2};
use wuli::{RigidBody, World};

use crate::ZERO_GRAVITY;

/// Two balls hit opposite ends of a free bar from opposite sides, so it spins
/// in place: the balls' momenta cancel, but their angular momenta add up
pub fn spinning_bar() -> World {
    let mut world = World::new(ZERO_GRAVITY);
    world.add_body(RigidBody::rectangle(
        DVec2::ZERO,
        DVec2::ZERO,
        1.0,
        dvec2(1.0, 0.05),
        0.0,
        0.0,
    ));
    world.add_body(RigidBody::circle(
        dvec2(0.8, -2.0),
        dvec2(0.0, 4.0),
        1.0,
        0.1,
    ));
    world.add_body(RigidBody::circle(
        dvec2(-0.8, 2.0),
        dvec2(0.0, -4.0),
        1.0,
        0.1,
    ));
    world
}
