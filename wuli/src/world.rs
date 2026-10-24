use glam::{DVec2, dvec2};
use slotmap::SlotMap;

use crate::{
    HandleNotFound,
    body::{BodyHandle, RigidBody},
    collision,
};

#[derive(Clone)]
pub struct World {
    pub gravity: DVec2, // e.g. $g =-9.81m/s^2$
    pub bodies: SlotMap<BodyHandle, RigidBody>,
}

impl World {
    pub fn new(gravity: f64) -> Self {
        Self {
            gravity: dvec2(0.0, gravity),
            bodies: Default::default(),
        }
    }

    pub fn add_body(&mut self, body: RigidBody) -> BodyHandle {
        self.bodies.insert(body)
    }

    pub fn step(&mut self, dt: f64) {
        for (_, b) in &mut self.bodies {
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
        self.bodies.keys().collect()
    }

    pub fn body(&self, handle: BodyHandle) -> Result<&RigidBody, HandleNotFound> {
        self.bodies.get(handle).ok_or(HandleNotFound)
    }

    pub fn body_mut(&mut self, handle: BodyHandle) -> Result<&mut RigidBody, HandleNotFound> {
        self.bodies.get_mut(handle).ok_or(HandleNotFound)
    }

    pub fn bodies_mut<const N: usize>(
        &mut self,
        handles: [BodyHandle; N],
    ) -> Result<[&mut RigidBody; N], HandleNotFound> {
        self.bodies.get_disjoint_mut(handles).ok_or(HandleNotFound)
    }
}
