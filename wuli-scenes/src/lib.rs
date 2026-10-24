pub const NORMAL_GRAVITY: f64 = -9.81;
pub const ZERO_GRAVITY: f64 = 0.0;

pub mod projectile;
pub use projectile::projectile_a;

pub mod collision;
pub use collision::colliding_circles_a;
use wuli::World;

pub fn available() -> Vec<(String, fn() -> World)> {
    vec![
        ("Projectile".to_string(), projectile_a),
        ("Colliding Circles".to_string(), colliding_circles_a),
    ]
}

pub fn empty() -> World {
    World::new(NORMAL_GRAVITY)
}
