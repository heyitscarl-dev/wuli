use glam::DVec2;
use wuli::{body::Shape, *};

const NORMAL_GRAVITY: f64 = -9.81;

pub fn projectile(vx0: f64) -> World {
    let shape = Shape::Circle { radius: 0.1 };
    let mass = 10.0;
    let inertia = shape.inertia(mass);
    let body = RigidBody {
        shape: Shape::Circle { radius: 0.1 },
        position: DVec2 { x: 0.0, y: 0.0 },
        angle: 0.0,
        velocity: DVec2 { x: vx0, y: 0.0 },
        angular_velocity: 0.0,
        inv_mass: 1.0 / mass, // 1 / 10kg = 0.1kg
        inv_inertia: 1.0 / inertia,
    };

    World {
        gravity: DVec2 {
            x: 0.0,
            y: NORMAL_GRAVITY,
        },
        bodies: vec![body],
    }
}

pub fn projectile_2() -> World {
    let shape = Shape::Circle { radius: 0.1 };
    let mass = 10.0;
    let inertia = shape.inertia(mass);
    let body1 = RigidBody {
        shape,
        position: DVec2 { x: 0.0, y: 0.0 },
        angle: 0.0,
        velocity: DVec2 { x: 2.5, y: -4.25 },
        angular_velocity: 0.0,
        inv_mass: 1.0 / mass, // 1 / 10kg = 0.1kg
        inv_inertia: 1.0 / inertia,
    };

    let shape = Shape::Rectangle {
        half_extents: DVec2 { x: 1.0, y: 0.2 },
    };
    let mass = 50.0;
    let inertia = shape.inertia(mass);
    let body2: RigidBody = RigidBody {
        shape,
        position: DVec2 { x: 2.5, y: -5.0 },
        angle: 0.0,
        velocity: DVec2 { x: 0.0, y: 0.0 },
        angular_velocity: -0.1,
        inv_mass: 1.0 / mass,
        inv_inertia: 1.0 / inertia,
    };

    World {
        gravity: DVec2 { x: 0.0, y: 0.0 },
        bodies: vec![body1, body2],
    }
}
