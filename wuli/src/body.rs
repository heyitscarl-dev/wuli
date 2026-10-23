use glam::{DVec2, dvec2};

#[derive(Clone)]
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
}

#[derive(Clone)]
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
