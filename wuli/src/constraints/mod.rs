use slotmap::new_key_type;

use crate::{
    World,
    body::BodyHandle,
    collision::narrow::Collision,
    constraints::{self, distance::DistanceConstraint, pin::PinConstraint},
};

// Constraint types
pub mod contact;
pub mod distance;
pub mod pin;

pub mod xpbd;

new_key_type! { pub struct ConstraintHandle; }

#[derive(Debug, Clone)]
pub struct Constraint {
    pub kind: ConstraintKind,
    pub soft: bool,
}

impl Constraint {
    pub fn to_string(&self) -> String {
        self.kind.to_string()
    }
}

#[derive(Debug, Clone)]
pub enum ConstraintKind {
    Distance { constraint: DistanceConstraint },
    Contact { collision: Collision },
    Pin { constraint: PinConstraint },
}

impl ConstraintKind {
    pub fn solve_positions(&mut self, world: &mut World, h: f64) {
        match self {
            Self::Contact { collision } => {
                let _ = constraints::contact::solve_contact_positions(world, collision, h);
            }
            Self::Distance { constraint } => {
                let _ = constraints::distance::solve_distance_positions(world, constraint, h);
            }
            Self::Pin { constraint } => {
                let _ = constraints::pin::solve_pin_positions(world, constraint, h);
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
            Self::Pin { constraint } => {
                let _ = constraints::pin::solve_pin_velocities(world, constraint, h);
            }
        }
    }

    pub fn bodies(&self) -> Vec<BodyHandle> {
        match self {
            Self::Contact { collision } => vec![collision.a, collision.b],
            Self::Distance { constraint } => vec![constraint.a, constraint.b],
            Self::Pin { constraint } => vec![constraint.body],
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Contact { .. } => "Contact".to_string(),
            Self::Distance { .. } => "Distance".to_string(),
            Self::Pin { .. } => "Pin".to_string(),
        }
    }
}
