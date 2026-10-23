use crate::{RigidBody, World, collision::{Collides, aabb::AxisAlignedBoundingBox}};

impl Collides for (AxisAlignedBoundingBox, AxisAlignedBoundingBox) {
    fn collides(&self) -> bool {
        let is_a_left_b = self.0.max.x < self.1.min.x;
        let is_a_right_b = self.0.min.x > self.1.max.x;
        let is_a_above_b = self.0.min.y > self.1.max.y;
        let is_a_below_b = self.0.max.y < self.1.min.y;
        let no_collision = is_a_left_b || is_a_right_b || is_a_above_b || is_a_below_b;
        !no_collision
    }
}

pub fn get_possible_collisions(world: &World) -> Vec<(&RigidBody, &RigidBody)> {
    let mut pairs: Vec<(&RigidBody, &RigidBody)> = Vec::new();

    // FIXME: Switch to a (e.g.) grid based approach to solve O(n²)

    let bodies = world.bodies();
    for a in bodies {
        for b in bodies {
            if a == b { continue; }

            let a_box = AxisAlignedBoundingBox::from_body(a);
            let b_box = AxisAlignedBoundingBox::from_body(b);
            if !(a_box, b_box).collides() {
                continue;
            }

            pairs.push((a, b));
        }
    }

    pairs
}