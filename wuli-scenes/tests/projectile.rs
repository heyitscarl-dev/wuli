use glam::DVec2;

#[test]
fn projectile_matches_analytic() {
    let mut w = wuli_scenes::projectile(5.0);
    // given: x(t) = x_0 + vx_0t
    // given: y(t) = y_0 + vy_0t + 1/2(gt^2)

    let dt = 1.0 / 240.0; // 240 Hz
    for _ in 0..240 {
        w.step(dt);
    }

    // compare against t = 1s

    let expected = DVec2 {
        x: 0.0 + 5.0 * 1.0,
        y: 0.0 + 0.0 * 1.0 + 0.5 * (-9.81) * 1.0 * 1.0,
    };

    let actual = w.bodies.first().expect("Projectile non-existent").position;
    let error = (expected - actual).length();

    assert!(error < 0.03, "Error too large: {error}");
}
