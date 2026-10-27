use glam::dvec2;

use crate::{HandleNotFound, World, body::BodyHandle, constraints::xpbd::apply_correction};

#[derive(Debug, Clone)]
pub struct DistanceConstraint {
    pub a: BodyHandle,
    pub b: BodyHandle,
    pub distance: f64,
    pub compliance: f64,
    pub lambda: f64,
}

pub fn solve_distance_positions(
    world: &mut World,
    constraint: &mut DistanceConstraint,
    h: f64,
) -> Result<(), HandleNotFound> {
    let a = world.body(constraint.a)?;
    let b = world.body(constraint.b)?;

    let delta = b.position - a.position;
    let normal = delta.normalize();
    let error = delta.length() - constraint.distance;

    if error < 1e-6 { return Ok(()); }

    apply_correction(
        world,
        constraint.a,
        constraint.b,
        normal,
        error,
        dvec2(0.0, 0.0), // skip anchor points for now, make all constraints between body centers
        dvec2(0.0, 0.0),
        constraint.compliance,
        &mut constraint.lambda,
        h,
    )
}