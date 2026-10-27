use glam::dvec2;
use wuli::{RigidBody, World};

use crate::NORMAL_GRAVITY;

fn projectiles(dx: f64, dy: f64, vxa: f64, vxb: f64, mr: f64) -> World {
    let origin = dvec2(0.0, 0.0);
    let body_a = RigidBody::circle(
        origin - dvec2(dx * 0.5, dy * 0.5),
        dvec2(vxa, 0.0),
        1.0,
        0.1,
    );
    let body_b = RigidBody::circle(
        origin + dvec2(dx * 0.5, dy * 0.5),
        dvec2(vxb, 0.0),
        1.0 / mr,
        0.1,
    );
    let gravity = dvec2(0.0, NORMAL_GRAVITY);
    let mut world = World {
        gravity,
        ..Default::default()
    };
    world.bodies.insert(body_a);
    world.bodies.insert(body_b);
    world
}

fn projectile(vx: f64) -> World {
    let origin = dvec2(0.0, 0.0);
    let body = RigidBody::circle(origin, dvec2(vx, 0.0), 1.0, 0.1);
    let gravity = dvec2(0.0, NORMAL_GRAVITY);
    let mut world = World {
        gravity,
        ..Default::default()
    };
    world.bodies.insert(body);
    world
}

pub fn projectiles_a() -> World {
    projectiles(5.0, 0.0, 2.0, -2.0, 1.0)
}

pub fn projectile_a() -> World {
    projectile(5.0)
}
