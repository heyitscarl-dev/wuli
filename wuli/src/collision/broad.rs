use crate::{World, body::BodyHandle, collision::{CouldCollide, aabb::AxisAlignedBoundingBox}};

impl CouldCollide for (AxisAlignedBoundingBox, AxisAlignedBoundingBox) {
    fn could_collide(&self) -> bool {
        let is_a_left_b = self.0.max.x < self.1.min.x;
        let is_a_right_b = self.0.min.x > self.1.max.x;
        let is_a_above_b = self.0.min.y > self.1.max.y;
        let is_a_below_b = self.0.max.y < self.1.min.y;
        let no_collision = is_a_left_b || is_a_right_b || is_a_above_b || is_a_below_b;
        !no_collision
    }
}

pub fn get_possible_collisions(world: &World) -> Vec<(BodyHandle, BodyHandle)> {
    let mut pairs: Vec<(BodyHandle, BodyHandle)> = Vec::new();

    // FIXME: Switch to a (e.g.) grid based approach to solve O(n²)

    let handles = world.handles();
    for (i, ha) in handles.iter().enumerate() {
        for hb in &handles[i + 1..] {
            let a = world.body(*ha);
            let b = world.body(*hb);
            let a_box = AxisAlignedBoundingBox::from_body(a);
            let b_box = AxisAlignedBoundingBox::from_body(b);
            if !(a_box, b_box).could_collide() {
                continue;
            }

            pairs.push((*ha, *hb));
        }
    }

    pairs
}