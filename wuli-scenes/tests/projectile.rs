use wuli::world::SUBSTEPS;
use wuli_scenes::{NORMAL_GRAVITY, projectile_a};

/// Runs projectile_a for 1 s with `steps` steps and returns the final position.
fn simulate(steps: usize) -> (f64, f64) {
    let mut world = projectile_a();
    let delta = 1.0 / steps as f64;
    for _ in 0..steps {
        world.step(delta);
    }
    let p = world
        .bodies
        .values()
        .next()
        .expect("expected projectile in scene")
        .position;
    (p.x, p.y)
}

#[test]
fn test_projectile_a() {
    // Exact: x(t) = vx·t = 5.0, y(t) = y0 + vy0·t + ½·g·t² = ½·g  (y0 = vy0 = 0)
    let expected_x = 5.0;
    let expected_y = 0.5 * NORMAL_GRAVITY;

    let steps = 240;
    let dt = 1.0 / steps as f64;
    let (x1, y1) = simulate(steps);
    let (x2, y2) = simulate(steps * 2);

    // Constant velocity: any integrator should get x exact up to rounding.
    assert!(
        (expected_x - x1).abs() < 1e-9,
        "x error {:e} at {steps} steps",
        expected_x - x1
    );
    assert!(
        (expected_x - x2).abs() < 1e-9,
        "x error {:e} at {} steps",
        expected_x - x2,
        steps * 2
    );

    // First-order (semi-implicit) Euler: |y error| = ½·|g|·h exactly for constant g,
    // where h = dt / SUBSTEPS is the size of each integration step.
    let err1 = expected_y - y1;
    let predicted = 0.5 * NORMAL_GRAVITY.abs() * dt / SUBSTEPS as f64;
    assert!(
        (err1.abs() - predicted).abs() < 1e-6,
        "y error {err1:e}, predicted magnitude {predicted:e} for first-order Euler"
    );

    // Halving dt should halve the error (first order).
    let ratio = (expected_y - y2) / err1;
    assert!(
        (ratio - 0.5).abs() < 1e-3,
        "y error ratio {ratio}, expected 0.5 (first-order)"
    );
}
