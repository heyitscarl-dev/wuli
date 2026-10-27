use glam::dvec2;
use wuli::{
    RigidBody, World,
    constraints::{Constraint, ConstraintKind, distance::DistanceConstraint},
};

use crate::NORMAL_GRAVITY;

pub fn simple_pendulum() -> World {
    let anchor = RigidBody::circle(dvec2(0.0, 0.0), dvec2(0.0, 0.0), 0.0, 0.1);
    let bob = RigidBody::circle(dvec2(-1.0, 0.0), dvec2(0.0, 0.0), 1.0, 0.1);
    let mut world = World::new(NORMAL_GRAVITY);
    let a = world.add_body(anchor);
    let b = world.add_body(bob);
    let distance = Constraint {
        kind: ConstraintKind::Distance {
            constraint: DistanceConstraint {
                a,
                b,
                distance: 1.0,
                compliance: 0.0,
                lambda: 0.0,
            },
        },
        soft: false,
    };
    world.add_constraint(distance);
    world
}

pub fn double_pendulum() -> World {
    let anchor = RigidBody::circle(dvec2(0.0, 0.0), dvec2(0.0, 0.0), 0.0, 0.1);
    let bob1 = RigidBody::circle(dvec2(-1.0, 0.0), dvec2(0.0, 0.0), 1.0, 0.1);
    let bob2 = RigidBody::circle(dvec2(-2.0, 0.0), dvec2(0.0, 0.0), 1.0, 0.1);
    let mut world = World::new(NORMAL_GRAVITY);
    let a = world.add_body(anchor);
    let b = world.add_body(bob1);
    let c = world.add_body(bob2);
    let distance1 = Constraint {
        kind: ConstraintKind::Distance {
            constraint: DistanceConstraint {
                a,
                b,
                distance: 1.0,
                compliance: 0.0,
                lambda: 0.0,
            },
        },
        soft: false,
    };
    let distance2 = Constraint {
        kind: ConstraintKind::Distance {
            constraint: DistanceConstraint {
                a: b,
                b: c,
                distance: 1.0,
                compliance: 0.0,
                lambda: 0.0,
            },
        },
        soft: false,
    };
    world.add_constraint(distance1);
    world.add_constraint(distance2);
    world
}
