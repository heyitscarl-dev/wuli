use glam::DVec2;

use crate::{
    body::{BodyHandle, RigidBody},
    collision,
};

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

            let force = 0.0; // FIXME: Add force accumulation
            let torque = 0.0;

            b.velocity += (self.gravity + force) * dt; // semi-implicit Euler (integration)
            b.angular_velocity += torque * dt;
            b.position += b.velocity * dt;
            b.angle += b.angular_velocity * dt;
        }

        let collision_pairs = collision::broad::get_possible_collisions(self);
        for pair in collision_pairs {
            let collision = collision::narrow::get_collision(self, pair.0, pair.1);
            if collision.is_none() {
                continue;
            }
            let collision = collision.unwrap();
            collision::solve::handle_collision(self, collision);
        }
    }

    pub fn handles(&self) -> Vec<BodyHandle> {
        self.bodies
            .iter()
            .enumerate()
            .map(|(i, _)| i)
            .map(|i| BodyHandle(i))
            .collect()
    }

    pub fn body(&self, handle: BodyHandle) -> &RigidBody {
        &self.bodies[handle.0]
    }

    pub fn body_mut(&mut self, handle: BodyHandle) -> &mut RigidBody {
        &mut self.bodies[handle.0]
    }
}
