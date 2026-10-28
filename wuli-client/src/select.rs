use glam::dvec2;
use macroquad::input::mouse_position;
use wuli::{World, body::BodyHandle, constraints::ConstraintHandle};

use crate::view::View;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Selection {
    None,
    Body(BodyHandle),
    Constraint(ConstraintHandle),
    Many(Vec<Selection>)
}

impl Selection {
    pub fn from_mouse(world: &World, view: &View) -> Self {
        Self::body_from_mouse(world, view)
            .or_else(|| Self::constraint_from_mouse())
            .unwrap_or(Self::None)
    }
    
    pub fn body_from_mouse(world: &World, view: &View) -> Option<Self> {
        let mouse = mouse_position();
        let mouse = view.camera()
            .screen_to_world(macroquad::math::vec2(mouse.0, mouse.1));
        let mouse = dvec2(mouse.x as f64, mouse.y as f64);
        world.body_at(mouse)
            .map(|handle| Self::Body(handle))
    }

    pub fn constraint_from_mouse() -> Option<Self> {
        // TODO: Add constraint selection
        None
    }

    pub fn contains_body(&self, handle: BodyHandle) -> bool {
        match self {
            Selection::None | Selection::Constraint(_) => false,
            Selection::Body(self_handle) => *self_handle == handle,
            Selection::Many(selections) => selections.iter().any(|s|s.contains_body(handle))
        }
    }

    pub fn contains_constraint(&self, handle: ConstraintHandle) -> bool {
        match self {
            Selection::None | Selection::Body(_) => false,
            Selection::Constraint(self_handle) => *self_handle == handle,
            Selection::Many(selections) => selections.iter().any(|s|s.contains_constraint(handle))
        }
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Selection::None)
    }

    pub fn removed(self, other: &Selection) -> Selection {
        match (self.clone(), other) {
            (Selection::None, _) => {},
            (Selection::Body(handle), Selection::Body(other_handle)) => {
                if handle == *other_handle {
                    return Selection::None;
                }
            },
            (Selection::Constraint(handle), Selection::Constraint(other_handle)) => {
                if handle == *other_handle {
                    return Selection::None;
                }
            },
            (Selection::Many(selections), _) => {
                let mut selections = selections.clone();
                selections.retain(|selection| selection != other);

                if selections.is_empty() {
                    return Selection::None;
                } else if selections.len() == 1 {
                    return selections.pop().unwrap();
                }
                return Selection::Many(selections);
            },
            _ => {},
        }
        return self;
    }

    pub fn appended(self, other: Selection) -> Selection {
        match self.clone() {
            Selection::None => other,
            Selection::Body(_) | Selection::Constraint(_) => {
                Selection::Many(vec![self, other])
            },
            Selection::Many(selections) => {
                Selection::Many(vec![selections, vec![other]].into_iter().flatten().collect())
            },
        }
    }

    pub fn delete(&self, world: &mut wuli::World) {
        match self {
            Selection::None => {},
            Selection::Body(handle) => {
                let _ = world.clean_remove_body(*handle);
            },
            Selection::Constraint(handle) => {
                let _ = world.remove_constraint(*handle);
            },
            Selection::Many(selections) => {
                for selection in selections {
                    selection.delete(world);
                }
            }
        }
    }
}