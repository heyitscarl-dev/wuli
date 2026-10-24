use crate::{
    World,
    body::BodyHandle,
    collision::{CouldCollide, aabb::AxisAlignedBoundingBox},
};

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
            let a = world.body(*ha).expect("expected body to exist");
            let b = world.body(*hb).expect("expected body to exist");
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RigidBody;
    use glam::{DVec2, dvec2};

    fn aabb(min: (f64, f64), max: (f64, f64)) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox {
            min: dvec2(min.0, min.1),
            max: dvec2(max.0, max.1),
        }
    }

    fn ball(x: f64, y: f64) -> RigidBody {
        RigidBody::circle(dvec2(x, y), DVec2::ZERO, 1.0, 0.5)
    }

    /// Builds a world from `bodies`, returning the handles in insertion order.
    fn world(gravity: DVec2, bodies: Vec<RigidBody>) -> (World, Vec<BodyHandle>) {
        let mut w = World {
            gravity,
            bodies: Default::default(),
        };
        let handles = bodies.into_iter().map(|b| w.bodies.insert(b)).collect();
        (w, handles)
    }

    #[test]
    fn overlapping_boxes_could_collide() {
        assert!((aabb((0.0, 0.0), (2.0, 2.0)), aabb((1.0, 1.0), (3.0, 3.0))).could_collide());
    }

    #[test]
    fn contained_box_could_collide() {
        assert!((aabb((0.0, 0.0), (4.0, 4.0)), aabb((1.0, 1.0), (2.0, 2.0))).could_collide());
    }

    #[test]
    fn touching_boxes_could_collide() {
        assert!((aabb((0.0, 0.0), (1.0, 1.0)), aabb((1.0, 0.0), (2.0, 1.0))).could_collide());
    }

    #[test]
    fn separated_boxes_cannot_collide() {
        let a = (0.0, 0.0);
        let b = (1.0, 1.0);
        // left, right, above, below
        assert!(!(aabb(a, b), aabb((2.0, 0.0), (3.0, 1.0))).could_collide());
        assert!(!(aabb(a, b), aabb((-3.0, 0.0), (-2.0, 1.0))).could_collide());
        assert!(!(aabb(a, b), aabb((0.0, 2.0), (1.0, 3.0))).could_collide());
        assert!(!(aabb(a, b), aabb((0.0, -3.0), (1.0, -2.0))).could_collide());
    }

    #[test]
    fn overlap_on_one_axis_only_cannot_collide() {
        assert!(!(aabb((0.0, 0.0), (1.0, 1.0)), aabb((0.5, 5.0), (1.5, 6.0))).could_collide());
    }

    #[test]
    fn no_pairs_for_distant_bodies() {
        let (w, _) = world(
            DVec2::ZERO,
            vec![ball(0.0, 0.0), ball(10.0, 0.0), ball(0.0, 10.0)],
        );
        assert!(get_possible_collisions(&w).is_empty());
    }

    #[test]
    fn no_self_pairs() {
        let (w, _) = world(DVec2::ZERO, vec![ball(0.0, 0.0)]);
        assert!(get_possible_collisions(&w).is_empty());
    }

    #[test]
    fn overlapping_bodies_are_paired() {
        let (w, h) = world(
            DVec2::ZERO,
            vec![ball(0.0, 0.0), ball(0.8, 0.0), ball(10.0, 0.0)],
        );
        let pairs = get_possible_collisions(&w);
        assert!(!pairs.is_empty());
        assert!(
            pairs
                .iter()
                .all(|&(a, b)| { (a, b) == (h[0], h[1]) || (a, b) == (h[1], h[0]) })
        );
    }

    #[test]
    fn each_pair_reported_once() {
        let (w, _) = world(DVec2::ZERO, vec![ball(0.0, 0.0), ball(0.8, 0.0)]);
        let pairs = get_possible_collisions(&w);
        assert_eq!(
            pairs.len(),
            1,
            "pair (0, 1) was reported {} times",
            pairs.len()
        );
    }
}
