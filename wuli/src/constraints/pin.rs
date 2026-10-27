use glam::DVec2;

use crate::{
    HandleNotFound, World, body::BodyHandle, constraints::xpbd::{apply_single_body_correction, scalar_cross_product},
};

#[derive(Debug, Clone)]
pub struct PinConstraint {
    pub body: BodyHandle,
    pub local_anchor: DVec2,
    pub target: DVec2,
    pub compliance: f64,
    pub damping: f64,
    pub angular_damping: f64,
    pub lambda: f64,
}

pub fn solve_pin_positions(
    world: &mut World,
    constraint: &mut PinConstraint,
    h: f64,
) -> Result<(), HandleNotFound> {
    let body = world.body(constraint.body)?;

    let r = constraint.local_anchor.rotate_angle(body.angle);
    let delta = body.position + r - constraint.target;
    let normal = delta.normalize();
    let error = delta.length();

    if error < 1e-6 {
        return Ok(());
    }

    apply_single_body_correction(
        world,
        constraint.body,
        normal,
        -error,
        r,
        constraint.compliance,
        &mut constraint.lambda,
        h,
    )
}

/// Damp the anchor point's velocity, so a dragged body settles instead of bouncing on the spring
pub fn solve_pin_velocities(
    world: &mut World,
    constraint: &PinConstraint,
    h: f64,
) -> Result<(), HandleNotFound> {
    let body = world.body_mut(constraint.body)?;

    if body.is_static() {
        return Ok(());
    }

    let r = constraint.local_anchor.rotate_angle(body.angle);
    let v = body.velocity + body.angular_velocity * r.perp(); // velocity of the anchor point
    let dv = -v * (constraint.damping * h).min(1.0);
    if let Some(normal) = dv.try_normalize() {
        let w = body.inv_mass
            + body.inv_inertia * scalar_cross_product(r, normal) * scalar_cross_product(r, normal);
        let p = dv / w; // impulse

        body.velocity += p * body.inv_mass;
        body.angular_velocity += body.inv_inertia * scalar_cross_product(r, p);
    }
    body.angular_velocity *= 1.0 - (constraint.angular_damping * h).min(1.0);

    Ok(())
}