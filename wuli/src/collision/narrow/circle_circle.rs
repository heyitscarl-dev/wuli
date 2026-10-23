use glam::DVec2;

use crate::{World, body::BodyHandle, collision::narrow::Collision};

pub fn get_circle_circle_collision(
    world: &World,
    body_a: BodyHandle,
    body_b: BodyHandle,
    ra: f64,
    rb: f64,
) -> Option<Collision> {
    let a = world.body(body_a).position;
    let b = world.body(body_b).position;
    if (ra + rb) < (a - b).length() {
        return None;
    }
    let normal = (b - a).try_normalize().unwrap_or(DVec2::Y); // concentric: any direction separates them
    let depth = ra + rb - (a - b).length();
    let contact = a + normal * ra;
    Some(Collision {
        a: body_a,
        b: body_b,
        normal,
        depth,
        contact,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RigidBody, collision::narrow::get_collision};
    use glam::{DVec2, dvec2};

    const EPS: f64 = 1e-12;

    fn two_balls(a: DVec2, ra: f64, b: DVec2, rb: f64) -> World {
        World {
            gravity: DVec2::ZERO,
            bodies: vec![
                RigidBody::circle(a, DVec2::ZERO, 1.0, ra),
                RigidBody::circle(b, DVec2::ZERO, 1.0, rb),
            ],
        }
    }

    #[test]
    fn separated_circles_do_not_collide() {
        let w = two_balls(dvec2(0.0, 0.0), 0.5, dvec2(2.0, 0.0), 0.5);
        assert!(get_collision(&w, BodyHandle(0), BodyHandle(1)).is_none());
    }

    #[test]
    fn overlapping_circles_report_normal_depth_and_contact() {
        let w = two_balls(dvec2(0.0, 0.0), 1.0, dvec2(1.5, 0.0), 1.0);
        let c = get_collision(&w, BodyHandle(0), BodyHandle(1)).expect("expected collision");
        assert!(
            (c.normal - dvec2(1.0, 0.0)).length() < EPS,
            "normal should point from a to b"
        );
        assert!((c.depth - 0.5).abs() < EPS);
        assert!((c.contact - dvec2(1.0, 0.0)).length() < EPS);
        assert!(c.a == BodyHandle(0) && c.b == BodyHandle(1));
    }

    #[test]
    fn normal_is_unit_length_on_diagonal() {
        let w = two_balls(dvec2(0.0, 0.0), 1.0, dvec2(1.0, 1.0), 1.0);
        let c = get_collision(&w, BodyHandle(0), BodyHandle(1)).expect("expected collision");
        assert!((c.normal.length() - 1.0).abs() < EPS);
        assert!((c.depth - (2.0 - 2f64.sqrt())).abs() < EPS);
    }

    #[test]
    fn swapping_order_flips_normal() {
        let w = two_balls(dvec2(0.0, 0.0), 1.0, dvec2(1.5, 0.0), 1.0);
        let ab = get_collision(&w, BodyHandle(0), BodyHandle(1)).unwrap();
        let ba = get_collision(&w, BodyHandle(1), BodyHandle(0)).unwrap();
        assert!((ab.normal + ba.normal).length() < EPS);
        assert!((ab.depth - ba.depth).abs() < EPS);
    }

    #[test]
    fn different_radii() {
        let w = two_balls(dvec2(0.0, 0.0), 2.0, dvec2(0.0, 2.5), 1.0);
        let c = get_collision(&w, BodyHandle(0), BodyHandle(1)).expect("expected collision");
        assert!((c.normal - dvec2(0.0, 1.0)).length() < EPS);
        assert!((c.depth - 0.5).abs() < EPS);
        assert!((c.contact - dvec2(0.0, 2.0)).length() < EPS);
    }

    #[test]
    fn concentric_circles_give_finite_normal() {
        let w = two_balls(dvec2(0.0, 0.0), 1.0, dvec2(0.0, 0.0), 1.0);
        let c = get_collision(&w, BodyHandle(0), BodyHandle(1)).expect("expected collision");
        assert!(
            c.normal.is_finite(),
            "normal is {:?} for concentric circles",
            c.normal
        );
    }

    #[test]
    fn non_circle_shapes_are_not_handled_yet() {
        let mut w = two_balls(dvec2(0.0, 0.0), 1.0, dvec2(0.5, 0.0), 1.0);
        w.bodies[1].shape = crate::body::Shape::Rectangle {
            half_extents: dvec2(1.0, 1.0),
        };
        assert!(get_collision(&w, BodyHandle(0), BodyHandle(1)).is_none());
    }
}
