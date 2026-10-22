use glam::DVec2;

use crate::body::RigidBody;

#[derive(Clone)]
pub struct World {
    pub gravity: DVec2, // e.g. $g =-9.81m/s^2$
    pub bodies: Vec<RigidBody>,
}

impl World {
    pub fn step(&mut self, dt: f64) {
        for b in &mut self.bodies {
            if b.inv_mass == 0.0 {
                continue;
            } // skip static bodies
            b.velocity += self.gravity * dt; // semi-implicit Euler (integration)
            b.position += b.velocity * dt;
        }
    }

    pub fn bodies(&self) -> &Vec<RigidBody> {
        &self.bodies
    }
}
