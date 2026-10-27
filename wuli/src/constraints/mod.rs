use crate::{
    HandleNotFound, World, collision::narrow::Collision, constraints::{self, distance::DistanceConstraint, xpbd::apply_correction},
};

pub mod xpbd;
pub mod contact;
pub mod distance;

#[derive(Debug, Clone)]
pub enum Constraint {
    Distance { constraint: DistanceConstraint },
    Contact { collision: Collision }
}

impl Constraint {
    pub fn solve_positions(&mut self, world: &mut World, h: f64) {
        match self {
            Self::Contact { collision } => {
                let _ = constraints::contact::solve_contact_positions(world, collision, h);
            },
            Self::Distance { constraint } => {
                let _ = constraints::distance::solve_distance_positions(world, constraint, h);
            }
        }
    }

    pub fn solve_velocities(&mut self, world: &mut World, h: f64) {
        match self {
            Self::Contact { collision } => {
                let _ = constraints::contact::solve_contact_velocities(world, collision, h);
            },
            Self::Distance { .. } => {
                // no op
            }
        }
    }
}

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
