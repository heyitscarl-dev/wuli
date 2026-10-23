use glam::{DVec2, dvec2};
use wuli::{RigidBody, World, body::BodyHandle, collision::{narrow::get_collision, solve::handle_collision}};

const EPS: f64 = 1e-12;

fn world(bodies: Vec<RigidBody>) -> World {
    World { gravity: DVec2::ZERO, bodies }
}

fn ball(x: f64, vx: f64, inv_mass: f64) -> RigidBody {
    RigidBody::circle(dvec2(x, 0.0), dvec2(vx, 0.0), inv_mass, 0.5)
}

fn momentum(w: &World) -> DVec2 {
    w.bodies.iter().filter(|b| b.inv_mass > 0.0).map(|b| b.velocity / b.inv_mass).sum()
}

fn kinetic_energy(w: &World) -> f64 {
    w.bodies.iter().filter(|b| b.inv_mass > 0.0).map(|b| 0.5 * b.velocity.length_squared() / b.inv_mass).sum()
}

fn resolve(w: &mut World) {
    let c = get_collision(w, BodyHandle(0), BodyHandle(1)).expect("expected collision");
    handle_collision(w, c);
}

#[test]
fn equal_masses_swap_velocities() {
    let mut w = world(vec![ball(0.0, 1.0, 1.0), ball(0.9, -3.0, 1.0)]);
    resolve(&mut w);
    assert!((w.bodies[0].velocity - dvec2(-3.0, 0.0)).length() < EPS);
    assert!((w.bodies[1].velocity - dvec2(1.0, 0.0)).length() < EPS);
}

#[test]
fn unequal_masses_conserve_momentum_and_energy() {
    // m_a = 1, m_b = 4
    let mut w = world(vec![ball(0.0, 2.0, 1.0), ball(0.9, -1.0, 0.25)]);
    let (p0, e0) = (momentum(&w), kinetic_energy(&w));
    resolve(&mut w);
    assert!((momentum(&w) - p0).length() < EPS, "momentum {:?} -> {:?}", p0, momentum(&w));
    assert!((kinetic_energy(&w) - e0).abs() < EPS, "energy {e0} -> {}", kinetic_energy(&w));
}

#[test]
fn unequal_masses_match_analytic_result() {
    // 1D elastic: v_a' = ((m_a - m_b) v_a + 2 m_b v_b) / (m_a + m_b), etc.
    let (ma, mb, va, vb) = (1.0, 4.0, 2.0, -1.0);
    let mut w = world(vec![ball(0.0, va, 1.0 / ma), ball(0.9, vb, 1.0 / mb)]);
    resolve(&mut w);
    let va_ = ((ma - mb) * va + 2.0 * mb * vb) / (ma + mb);
    let vb_ = ((mb - ma) * vb + 2.0 * ma * va) / (ma + mb);
    assert!((w.bodies[0].velocity.x - va_).abs() < EPS);
    assert!((w.bodies[1].velocity.x - vb_).abs() < EPS);
}

#[test]
fn separating_bodies_are_left_alone() {
    let mut w = world(vec![ball(0.0, -1.0, 1.0), ball(0.9, 1.0, 1.0)]);
    let before = w.bodies.clone();
    resolve(&mut w);
    assert!(w.bodies == before, "separating bodies should not be touched");
}

#[test]
fn penetration_is_resolved() {
    let mut w = world(vec![ball(0.0, 1.0, 1.0), ball(0.6, -1.0, 1.0)]);
    resolve(&mut w);
    let gap = (w.bodies[1].position - w.bodies[0].position).length();
    assert!((gap - 1.0).abs() < EPS, "centres should be r_a + r_b apart, got {gap}");
}

#[test]
fn penetration_split_by_inverse_mass() {
    // m_a = 1, m_b = 3 → a moves 3/4 of the depth, b moves 1/4
    let mut w = world(vec![ball(0.0, 1.0, 1.0), ball(0.6, -1.0, 1.0 / 3.0)]);
    resolve(&mut w);
    assert!((w.bodies[0].position.x - (-0.3)).abs() < EPS);
    assert!((w.bodies[1].position.x - 0.7).abs() < EPS);
}

#[test]
fn ball_bounces_off_static_ball() {
    let mut static_ball = ball(0.9, 0.0, 0.0);
    static_ball.inv_mass = 0.0;
    let mut w = world(vec![ball(0.0, 2.0, 1.0), static_ball]);
    resolve(&mut w);
    assert!((w.bodies[0].velocity - dvec2(-2.0, 0.0)).length() < EPS);
    assert_eq!(w.bodies[1].velocity, DVec2::ZERO);
    assert_eq!(w.bodies[1].position, dvec2(0.9, 0.0));
}
