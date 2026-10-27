use crate::{
    HandleNotFound, World,
    collision::narrow::{Collision, get_collision},
    constraints::xpbd::{apply_correction, scalar_cross_product},
};

pub fn solve_contact_positions(
    world: &mut World,
    collision: &mut Collision,
    h: f64,
) -> Result<(), HandleNotFound> {
    let Some(current) = get_collision(world, collision.a, collision.b) else {
        return Ok(()); // already separated
    };

    collision.normal = current.normal;
    collision.depth = current.depth;
    collision.contact = current.contact;

    let a = world.body(collision.a)?;
    let b = world.body(collision.b)?;

    apply_correction(
        world,
        collision.a,
        collision.b,
        collision.normal,
        -collision.depth,
        collision.contact - a.position,
        collision.contact - b.position,
        0.0,
        &mut collision.lambda,
        h,
    )
}

pub fn solve_contact_velocities(
    world: &mut World,
    collision: &mut Collision,
) -> Result<(), HandleNotFound> {
    let e = 1.0; // perfect elasticity

    let [a, b] = world.bodies_mut([collision.a, collision.b])?;
    let vn_before = collision.normal.dot(b.prev_velocity - a.prev_velocity);
    if vn_before >= 0.0 {
        return Ok(()); // skip bodies that are already seperating
    }
    let vn_after = collision.normal.dot(b.velocity - a.velocity);
    let v_target = -e * vn_before;
    let dv = v_target - vn_after;
    let r_a = collision.contact - a.position;
    let r_b = collision.contact - b.position;
    let wa = a.inv_mass
        + a.inv_inertia
            * scalar_cross_product(r_a, collision.normal)
            * scalar_cross_product(r_a, collision.normal);
    let wb = b.inv_mass
        + b.inv_inertia
            * scalar_cross_product(r_b, collision.normal)
            * scalar_cross_product(r_b, collision.normal);
    if wa + wb == 0.0 {
        return Ok(()); // skip collisions where both bodies are static
    }
    let p = dv / (wa + wb) * collision.normal; // impulse

    a.velocity -= p * a.inv_mass;
    b.velocity += p * b.inv_mass;
    a.angular_velocity -= a.inv_inertia * scalar_cross_product(r_a, p);
    b.angular_velocity += b.inv_inertia * scalar_cross_product(r_b, p);

    Ok(())
}
