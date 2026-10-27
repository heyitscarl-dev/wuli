use glam::dvec2;
use wuli::{RigidBody, World, constraints::Constraint};

use crate::NORMAL_GRAVITY;

pub fn simple_pendulum() -> World {
    let anchor = RigidBody::circle(
        dvec2(0.0, 0.0), dvec2(0.0, 0.0), 0.0, 0.1
    );
    let bob = RigidBody::circle(
        dvec2(-1.0, 0.0), dvec2(0.0, 0.0), 1.0, 0.1
    );
    let mut world = World::new(NORMAL_GRAVITY);
    let a = world.add_body(anchor);
    let b = world.add_body(bob);
    let distance = Constraint::Distance { a, b, distance: 1.0, stiffness: 1.0 };
    world.add_constraint(distance);
    world
}