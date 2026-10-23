use glam::{DVec2, dvec2};
use wuli::{RigidBody, World, body::BodyHandle};

const EPS: f64 = 1e-12;

fn ball(pos: DVec2, vel: DVec2) -> RigidBody {
    RigidBody::circle(pos, vel, 1.0, 0.1)
}

#[test]
fn empty_world_steps() {
    let mut w = World { gravity: dvec2(0.0, -9.81), bodies: vec![] };
    w.step(1.0 / 60.0);
    assert!(w.bodies.is_empty());
}

#[test]
fn zero_dt_changes_nothing() {
    let mut w = World { gravity: dvec2(0.0, -9.81), bodies: vec![ball(dvec2(1.0, 2.0), dvec2(3.0, 4.0))] };
    let before = w.bodies.clone();
    w.step(0.0);
    assert!(w.bodies == before);
}

#[test]
fn no_gravity_means_constant_velocity() {
    let mut w = World { gravity: DVec2::ZERO, bodies: vec![ball(DVec2::ZERO, dvec2(1.5, -2.0))] };
    for _ in 0..100 {
        w.step(0.01);
    }
    assert!((w.bodies[0].velocity - dvec2(1.5, -2.0)).length() < EPS);
    assert!((w.bodies[0].position - dvec2(1.5, -2.0)).length() < 1e-9);
}

#[test]
fn velocity_under_gravity_is_exact() {
    // Semi-implicit Euler integrates constant acceleration into velocity exactly: v = g·t
    let g = dvec2(0.0, -9.81);
    let mut w = World { gravity: g, bodies: vec![ball(DVec2::ZERO, DVec2::ZERO)] };
    for _ in 0..120 {
        w.step(1.0 / 120.0);
    }
    assert!((w.bodies[0].velocity - g).length() < 1e-9);
}

#[test]
fn static_bodies_do_not_move() {
    let mut s = ball(dvec2(5.0, 5.0), dvec2(1.0, 1.0));
    s.inv_mass = 0.0;
    let mut w = World { gravity: dvec2(0.0, -9.81), bodies: vec![s] };
    for _ in 0..60 {
        w.step(1.0 / 60.0);
    }
    assert_eq!(w.bodies[0].position, dvec2(5.0, 5.0));
}

#[test]
fn angular_velocity_integrates_angle() {
    let mut b = ball(DVec2::ZERO, DVec2::ZERO);
    b.angular_velocity = 2.0;
    let mut w = World { gravity: DVec2::ZERO, bodies: vec![b] };
    for _ in 0..50 {
        w.step(0.01);
    }
    assert!((w.bodies[0].angle - 1.0).abs() < 1e-9);
}

#[test]
fn handles_cover_every_body() {
    let w = World {
        gravity: DVec2::ZERO,
        bodies: vec![ball(DVec2::ZERO, DVec2::ZERO), ball(DVec2::X, DVec2::ZERO), ball(DVec2::Y, DVec2::ZERO)],
    };
    let h = w.handles();
    assert_eq!(h.len(), 3);
    for (i, handle) in h.into_iter().enumerate() {
        assert!(handle == BodyHandle(i));
        assert!(w.body(handle) == &w.bodies[i]);
    }
}

#[test]
fn body_mut_writes_through() {
    let mut w = World { gravity: DVec2::ZERO, bodies: vec![ball(DVec2::ZERO, DVec2::ZERO)] };
    w.body_mut(BodyHandle(0)).velocity = dvec2(7.0, 0.0);
    assert_eq!(w.bodies[0].velocity, dvec2(7.0, 0.0));
}

#[test]
fn step_resolves_head_on_collision() {
    let mut w = World {
        gravity: DVec2::ZERO,
        bodies: vec![ball(dvec2(-0.5, 0.0), dvec2(1.0, 0.0)), ball(dvec2(0.5, 0.0), dvec2(-1.0, 0.0))],
    };
    for _ in 0..200 {
        w.step(0.01);
    }
    assert!(w.bodies[0].velocity.x < 0.0 && w.bodies[1].velocity.x > 0.0, "balls should have bounced");
    assert!(w.bodies[0].position.x < w.bodies[1].position.x, "balls should not pass through each other");
}
