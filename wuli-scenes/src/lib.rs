pub const NORMAL_GRAVITY: f64 = -9.81;
pub const ZERO_GRAVITY: f64 = 0.0;

pub mod projectile;
pub use projectile::projectile_a;

pub mod collision;
pub use collision::colliding_circles_a;
pub use collision::colliding_rectangles_a;

pub mod momentum;
pub use momentum::{billiards_break, counting_pi, newtons_row};

pub mod rotation;
pub use rotation::spinning_bar;

pub mod obstacles;
pub use obstacles::galton_board;

use glam::DVec2;
use wuli::{RigidBody, World};

pub fn available() -> Vec<(String, fn() -> World)> {
    vec![
        ("Projectile".to_string(), projectile_a),
        ("Colliding Circles".to_string(), colliding_circles_a),
        ("Colliding Rectangles".to_string(), colliding_rectangles_a),
        ("Newton's Row".to_string(), newtons_row),
        ("Billiards Break".to_string(), billiards_break),
        ("Counting π".to_string(), counting_pi),
        ("Spinning Bar".to_string(), spinning_bar),
        ("Galton Board".to_string(), galton_board),
    ]
}

pub fn empty() -> World {
    World::new(NORMAL_GRAVITY)
}

/// Static rectangle, used for walls, floors and cushions
pub fn wall(position: DVec2, half_extents: DVec2) -> RigidBody {
    RigidBody::rectangle(position, DVec2::ZERO, 0.0, half_extents, 0.0, 0.0)
}
