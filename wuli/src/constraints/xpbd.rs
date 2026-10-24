use glam::DVec2;

use crate::{HandleNotFound, World, body::BodyHandle};

pub fn scalar_cross_product(a: DVec2, b: DVec2) -> f64 {
    a.x * b.y - a.y * b.x
}

/// Extended Position Based Dynamics Solver (XPBD)
pub fn apply_correction(
    world: &mut World,
    ha: BodyHandle,
    hb: BodyHandle, // body handles
    n: DVec2,       // normal / "direction"
    c: f64,         // error
    r_a: DVec2,
    r_b: DVec2,      // relative coordinates of attack
    compliance: f64, // "elasticity"; 0 = rigid
    lambda: &mut f64,
    h: f64, // substep size
) -> Result<(), HandleNotFound> {
    let [a, b] = world.bodies_mut([ha, hb])?;

    if a.inv_mass == 0.0 && b.inv_mass == 0.0 {
        return Ok(());
    } // skip two static objects

    let wa =
        a.inv_mass + a.inv_inertia * scalar_cross_product(r_a, n) * scalar_cross_product(r_a, n);
    let wb =
        b.inv_mass + b.inv_inertia * scalar_cross_product(r_b, n) * scalar_cross_product(r_b, n);
    let alpha = compliance / (h * h);
    let dlambda = (-c - alpha * *lambda) / (wa + wb + alpha);
    *lambda += dlambda;
    let p = dlambda * n;

    a.position -= p * a.inv_mass;
    b.position += p * b.inv_mass;
    a.angle -= a.inv_inertia * scalar_cross_product(r_a, p);
    b.angle += b.inv_inertia * scalar_cross_product(r_b, p);

    Ok(())
}
