use glam::{DVec2, dvec2};
use slotmap::new_key_type;

new_key_type! { pub struct BodyHandle; }

#[derive(PartialEq, Clone)]
pub struct RigidBody {
    pub position: DVec2,
    pub angle: f64,
    pub velocity: DVec2,
    pub angular_velocity: f64,
    pub inv_mass: f64,
    pub inv_inertia: f64,
    pub shape: Shape,
}

impl RigidBody {
    pub fn bounds(&self) -> (DVec2, DVec2) {
        match self.shape {
            Shape::Circle { radius: r } => {
                (self.position - dvec2(r, r), self.position + dvec2(r, r))
            }
            Shape::Rectangle { half_extents: h } => {
                let [a, b, c, d] = Shape::rect_corners(self.position, self.angle, h);
                let min = a.min(b).min(c).min(d);
                let max = a.max(b).max(c).max(d);
                (min, max)
            }
        }
    }

    pub fn circle(position: DVec2, velocity: DVec2, inv_mass: f64, radius: f64) -> Self {
        let shape = Shape::Circle { radius };
        let inv_inertia = 1.0 / shape.inertia(1.0 / inv_mass); // static (inv_mass = 0) → 1/∞ = 0
        Self {
            shape,
            position,
            velocity,
            inv_mass,
            angle: 0.0,
            angular_velocity: 0.0,
            inv_inertia,
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum Shape {
    Rectangle { half_extents: DVec2 },
    Circle { radius: f64 },
}

impl Shape {
    pub fn area(&self) -> f64 {
        match *self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { half_extents: h } => 4.0 * h.x * h.y,
        }
    }
    pub fn inertia(&self, mass: f64) -> f64 {
        match *self {
            Shape::Circle { radius } => 0.5 * mass * radius * radius,
            Shape::Rectangle { half_extents: h } => mass * (h.x * h.x + h.y * h.y) / 3.0,
        }
    }

    pub fn rect_corners(pos: DVec2, angle: f64, h: DVec2) -> [DVec2; 4] {
        let rot = DVec2::from_angle(angle); // (cos θ, sin θ)
        [
            DVec2::new(-h.x, -h.y),
            DVec2::new(h.x, -h.y),
            DVec2::new(h.x, h.y),
            DVec2::new(-h.x, h.y),
        ]
        .map(|local| pos + rot.rotate(local))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI, SQRT_2};

    const EPS: f64 = 1e-12;

    fn approx(a: DVec2, b: DVec2) -> bool {
        (a - b).length() < EPS
    }

    #[test]
    fn circle_area() {
        let s = Shape::Circle { radius: 2.0 };
        assert!((s.area() - 4.0 * PI).abs() < EPS);
    }

    #[test]
    fn rectangle_area() {
        let s = Shape::Rectangle {
            half_extents: dvec2(1.0, 2.0),
        };
        assert!((s.area() - 8.0).abs() < EPS);
    }

    #[test]
    fn circle_inertia() {
        // Solid disc: I = ½·m·r²
        let s = Shape::Circle { radius: 2.0 };
        assert!((s.inertia(3.0) - 6.0).abs() < EPS);
    }

    #[test]
    fn rectangle_inertia() {
        // I = m·(w² + h²)/12 with full width w = 2, height h = 4
        let s = Shape::Rectangle {
            half_extents: dvec2(1.0, 2.0),
        };
        assert!((s.inertia(3.0) - 3.0 * (4.0 + 16.0) / 12.0).abs() < EPS);
    }

    #[test]
    fn rect_corners_unrotated() {
        let c = Shape::rect_corners(dvec2(1.0, 1.0), 0.0, dvec2(2.0, 1.0));
        assert!(approx(c[0], dvec2(-1.0, 0.0)));
        assert!(approx(c[1], dvec2(3.0, 0.0)));
        assert!(approx(c[2], dvec2(3.0, 2.0)));
        assert!(approx(c[3], dvec2(-1.0, 2.0)));
    }

    #[test]
    fn rect_corners_rotated_quarter_turn() {
        // Rotating counter-clockwise by 90° maps local (x, y) to (-y, x)
        let c = Shape::rect_corners(DVec2::ZERO, FRAC_PI_2, dvec2(2.0, 1.0));
        assert!(approx(c[0], dvec2(1.0, -2.0)));
        assert!(approx(c[1], dvec2(1.0, 2.0)));
        assert!(approx(c[2], dvec2(-1.0, 2.0)));
        assert!(approx(c[3], dvec2(-1.0, -2.0)));
    }

    #[test]
    fn circle_bounds() {
        let b = RigidBody::circle(dvec2(1.0, -1.0), DVec2::ZERO, 1.0, 0.5);
        let (min, max) = b.bounds();
        assert!(approx(min, dvec2(0.5, -1.5)));
        assert!(approx(max, dvec2(1.5, -0.5)));
    }

    #[test]
    fn rotated_square_bounds() {
        // A unit-half-extent square rotated by 45° reaches √2 along both axes
        let mut b = RigidBody::circle(DVec2::ZERO, DVec2::ZERO, 1.0, 1.0);
        b.shape = Shape::Rectangle {
            half_extents: dvec2(1.0, 1.0),
        };
        b.angle = FRAC_PI_4;
        let (min, max) = b.bounds();
        assert!(approx(min, dvec2(-SQRT_2, -SQRT_2)));
        assert!(approx(max, dvec2(SQRT_2, SQRT_2)));
    }

    #[test]
    fn circle_constructor_sets_fields() {
        let b = RigidBody::circle(dvec2(1.0, 2.0), dvec2(3.0, 4.0), 0.5, 0.1);
        assert_eq!(b.position, dvec2(1.0, 2.0));
        assert_eq!(b.velocity, dvec2(3.0, 4.0));
        assert_eq!(b.inv_mass, 0.5);
        assert_eq!(b.angle, 0.0);
        assert_eq!(b.angular_velocity, 0.0);
        assert!(b.shape == Shape::Circle { radius: 0.1 });
    }

    #[test]
    fn circle_constructor_inv_inertia_is_inverse() {
        // mass 1, radius 0.1 → I = 0.005, so 1/I = 200
        let b = RigidBody::circle(DVec2::ZERO, DVec2::ZERO, 1.0, 0.1);
        assert!(
            (b.inv_inertia - 200.0).abs() < 1e-9,
            "inv_inertia = {}, expected 1/I = 200",
            b.inv_inertia
        );
    }
}
