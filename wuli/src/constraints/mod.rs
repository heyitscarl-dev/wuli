use crate::{
    HandleNotFound, World, collision::narrow::Collision, constraints::xpbd::apply_correction,
};

pub mod xpbd;

// TODO: Move!!
pub fn handle_collision(
    world: &mut World,
    collision: &mut Collision,
    h: f64,
) -> Result<(), HandleNotFound> {
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
