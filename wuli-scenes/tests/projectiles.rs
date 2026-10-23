use wuli::World;
use wuli_scenes::{NORMAL_GRAVITY, projectile::projectiles_a};

const DT: f64 = 1.0 / 240.0;

fn run(world: &mut World, seconds: f64) {
    for _ in 0..(seconds / DT).round() as usize {
        world.step(DT);
    }
}

fn momentum_x(w: &World) -> f64 {
    w.bodies.iter().map(|b| b.velocity.x / b.inv_mass).sum()
}

fn kinetic_energy_x(w: &World) -> f64 {
    w.bodies
        .iter()
        .map(|b| 0.5 * b.velocity.x * b.velocity.x / b.inv_mass)
        .sum()
}

#[test]
fn approaching_before_collision() {
    // Gap of 5 − 0.2 = 4.8 m closes at 4 m/s → contact at t = 1.2 s
    let mut w = projectiles_a();
    run(&mut w, 1.0);
    assert_eq!(w.bodies[0].velocity.x, 2.0);
    assert_eq!(w.bodies[1].velocity.x, -2.0);
}

#[test]
fn equal_masses_swap_velocities_after_collision() {
    let mut w = projectiles_a();
    run(&mut w, 2.0);
    assert!(
        (w.bodies[0].velocity.x + 2.0).abs() < 1e-9,
        "a vx = {}",
        w.bodies[0].velocity.x
    );
    assert!(
        (w.bodies[1].velocity.x - 2.0).abs() < 1e-9,
        "b vx = {}",
        w.bodies[1].velocity.x
    );
    assert!(
        w.bodies[0].position.x < w.bodies[1].position.x,
        "balls passed through each other"
    );
}

#[test]
fn horizontal_momentum_and_energy_conserved() {
    let mut w = projectiles_a();
    let (p0, e0) = (momentum_x(&w), kinetic_energy_x(&w));
    run(&mut w, 3.0);
    assert!(
        (momentum_x(&w) - p0).abs() < 1e-9,
        "p_x {p0} -> {}",
        momentum_x(&w)
    );
    assert!(
        (kinetic_energy_x(&w) - e0).abs() < 1e-9,
        "E_x {e0} -> {}",
        kinetic_energy_x(&w)
    );
}

#[test]
fn both_fall_identically() {
    // The collision is purely horizontal, so it must not disturb vertical motion
    let mut w = projectiles_a();
    run(&mut w, 2.0);
    let (a, b) = (&w.bodies[0], &w.bodies[1]);
    assert_eq!(a.position.y, b.position.y);
    assert_eq!(a.velocity.y, b.velocity.y);
    assert!((a.velocity.y - 2.0 * NORMAL_GRAVITY).abs() < 1e-9);
}
