use glam::DVec2;

use crate::{HandleNotFound, World, body::BodyHandle, constraints::xpbd::{apply_single_body_correction}};

#[derive(Debug, Clone)]
pub struct PinConstraint {
    pub body: BodyHandle,
    pub local_anchor: DVec2,
    pub target: DVec2,
    pub compliance: f64,
    pub lambda: f64,
}

pub fn solve_pin_positions(
    world: &mut World,
    constraint: &mut PinConstraint,
    h: f64,
) -> Result<(), HandleNotFound> {
    let body = world.body(constraint.body)?;
    
    let delta = (body.position + constraint.local_anchor) - constraint.target;
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
        constraint.local_anchor, // relative coordinates of attack
        constraint.compliance,
        &mut constraint.lambda,
        h,
    )
}