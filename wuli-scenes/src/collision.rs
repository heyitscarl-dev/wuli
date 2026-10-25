use glam::dvec2;
use wuli::{RigidBody, World};

use crate::NORMAL_GRAVITY;

pub fn colliding_circles_a() -> World {
    let body_a = RigidBody::circle(dvec2(-3.0, 0.0), dvec2(5.0, 0.0), 1.0, 0.1);
    let body_b = RigidBody::circle(dvec2(3.0, 0.0), dvec2(-5.0, 0.0), 1.0, 0.1);
    let mut world = World::new(NORMAL_GRAVITY);
    world.add_body(body_a);
    world.add_body(body_b);
    world
}

pub fn colliding_rectangles_a() -> World {
    let body_a = RigidBody::rectangle(
        dvec2(-0.5, 0.0),
        dvec2(1.0, 0.0),
        0.5,
        dvec2(0.5, 0.1),
        14.0,
        0.0,
    );
    let body_b = RigidBody::circle(dvec2(0.5, 0.0), dvec2(-1.0, 0.0), 0.5, 0.2);
    let mut world = World::new(NORMAL_GRAVITY);
    world.add_body(body_a);
    world.add_body(body_b);
    world
}
