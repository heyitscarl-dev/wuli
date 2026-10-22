use glam::DVec2;
use wuli::*;

const NORMAL_GRAVITY: f64 = -9.81;

pub fn projectile(vx0: f64) -> World {
    let body = RigidBody {
        position: DVec2 { x: 0.0, y: 0.0 },
        velocity: DVec2 { x: vx0, y: 0.0 },
        inv_mass: 0.1, // 1 / 10kg = 0.1kg
    };

    World {
        gravity: DVec2 {
            x: 0.0,
            y: NORMAL_GRAVITY,
        },
        bodies: vec![body],
    }
}
