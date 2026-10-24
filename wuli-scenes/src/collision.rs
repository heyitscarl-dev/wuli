use glam::dvec2;
use wuli::{RigidBody, World};

use crate::NORMAL_GRAVITY;

pub fn colliding_circles_a() -> World {
    let body_a = RigidBody::circle(
        dvec2(-3.0, 0.0), 
        dvec2(5.0, 0.0), 
        1.0, 0.1
    );
    let body_b = RigidBody::circle(
        dvec2(3.0, 0.0), 
        dvec2(-5.0, 0.0), 
        1.0, 0.1
    );
    let mut world = World::new(NORMAL_GRAVITY);
    world.add_body(body_a);
    world.add_body(body_b);
    world
}