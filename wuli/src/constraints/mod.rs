use slotmap::new_key_type;

use crate::{
    World,
    body::BodyHandle,
    collision::narrow::Collision,
    constraints::{self, distance::DistanceConstraint},
};

pub mod contact;
pub mod distance;
pub mod xpbd;

new_key_type! { pub struct ConstraintHandle; }

#[derive(Debug, Clone)]
pub enum Constraint {
    Distance { constraint: DistanceConstraint },
    Contact { collision: Collision },
}

impl Constraint {
    pub fn solve_positions(&mut self, world: &mut World, h: f64) {
        match self {
            Self::Contact { collision } => {
                let _ = constraints::contact::solve_contact_positions(world, collision, h);
            }
            Self::Distance { constraint } => {
                let _ = constraints::distance::solve_distance_positions(world, constraint, h);
            }
        }
    }

    pub fn solve_velocities(&mut self, world: &mut World, h: f64) {
        match self {
            Self::Contact { collision } => {
                let _ = constraints::contact::solve_contact_velocities(world, collision);
            }
            Self::Distance { .. } => {
                // no op
            }
        }
    }

    pub fn bodies(&self) -> Vec<BodyHandle> {
        match self {
            Self::Contact { collision } => vec![collision.a, collision.b],
            Self::Distance { constraint } => vec![constraint.a, constraint.b],
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Contact { .. } => "Contact".to_string(),
            Self::Distance { .. } => "Distance".to_string(),
        }
    }
}
