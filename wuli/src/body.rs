use glam::DVec2;

#[derive(Clone)]
pub struct RigidBody {
    pub position: DVec2,
    pub velocity: DVec2,
    pub inv_mass: f64,
}
