use glam::DVec2;

use crate::{World, body::BodyHandle, collision::narrow::Collision};

pub fn get_circle_circle_collision(
    world: &World,
    body_a: BodyHandle,
    body_b: BodyHandle,
    ra: f64,
    rb: f64,
) -> Option<Collision> {
    let a = world.body(body_a).expect("expected body to exist").position;
    let b = world.body(body_b).expect("expected body to exist").position;
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
        lambda: 0.0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RigidBody, collision::narrow::get_collision};
    use glam::{DVec2, dvec2};

    const EPS: f64 = 1e-12;

    fn two_balls(a: DVec2, ra: f64, b: DVec2, rb: f64) -> (World, [BodyHandle; 2]) {
        let mut w = World {
            gravity: DVec2::ZERO,
            ..Default::default()
        };
        let ha = w.bodies.insert(RigidBody::circle(a, DVec2::ZERO, 1.0, ra));
        let hb = w.bodies.insert(RigidBody::circle(b, DVec2::ZERO, 1.0, rb));
        (w, [ha, hb])
    }

    #[test]
    fn separated_circles_do_not_collide() {
        let (w, h) = two_balls(dvec2(0.0, 0.0), 0.5, dvec2(2.0, 0.0), 0.5);
        assert!(get_collision(&w, h[0], h[1]).is_none());
    }

    #[test]
    fn overlapping_circles_report_normal_depth_and_contact() {
        let (w, h) = two_balls(dvec2(0.0, 0.0), 1.0, dvec2(1.5, 0.0), 1.0);
        let c = get_collision(&w, h[0], h[1]).expect("expected collision");
        assert!(
            (c.normal - dvec2(1.0, 0.0)).length() < EPS,
            "normal should point from a to b"
        );
        assert!((c.depth - 0.5).abs() < EPS);
        assert!((c.contact - dvec2(1.0, 0.0)).length() < EPS);
        assert!(c.a == h[0] && c.b == h[1]);
    }

    #[test]
    fn normal_is_unit_length_on_diagonal() {
        let (w, h) = two_balls(dvec2(0.0, 0.0), 1.0, dvec2(1.0, 1.0), 1.0);
        let c = get_collision(&w, h[0], h[1]).expect("expected collision");
        assert!((c.normal.length() - 1.0).abs() < EPS);
        assert!((c.depth - (2.0 - 2f64.sqrt())).abs() < EPS);
    }

    #[test]
    fn swapping_order_flips_normal() {
        let (w, h) = two_balls(dvec2(0.0, 0.0), 1.0, dvec2(1.5, 0.0), 1.0);
        let ab = get_collision(&w, h[0], h[1]).unwrap();
        let ba = get_collision(&w, h[1], h[0]).unwrap();
        assert!((ab.normal + ba.normal).length() < EPS);
        assert!((ab.depth - ba.depth).abs() < EPS);
    }

    #[test]
    fn different_radii() {
        let (w, h) = two_balls(dvec2(0.0, 0.0), 2.0, dvec2(0.0, 2.5), 1.0);
        let c = get_collision(&w, h[0], h[1]).expect("expected collision");
        assert!((c.normal - dvec2(0.0, 1.0)).length() < EPS);
        assert!((c.depth - 0.5).abs() < EPS);
        assert!((c.contact - dvec2(0.0, 2.0)).length() < EPS);
    }

    #[test]
    fn concentric_circles_give_finite_normal() {
        let (w, h) = two_balls(dvec2(0.0, 0.0), 1.0, dvec2(0.0, 0.0), 1.0);
        let c = get_collision(&w, h[0], h[1]).expect("expected collision");
        assert!(
            c.normal.is_finite(),
            "normal is {:?} for concentric circles",
            c.normal
        );
    }
}
