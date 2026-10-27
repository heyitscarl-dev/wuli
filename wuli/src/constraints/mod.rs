use std::fmt::Display;

use slotmap::new_key_type;

use crate::{
    HandleNotFound, World, body::BodyHandle, collision::narrow::Collision, constraints::{self, distance::DistanceConstraint, xpbd::apply_correction},
};

pub mod xpbd;
pub mod contact;
pub mod distance;

new_key_type! { pub struct ConstraintHandle; }

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

    pub fn bodies(&self) -> Vec<BodyHandle> {
        match self {
            Self::Contact { collision } => vec![collision.a, collision.b],
            Self::Distance { constraint } => vec![constraint.a, constraint.b]
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Contact { .. } => format!("Contact"),
            Self::Distance { .. } => format!("Distance")
        }
    }
}