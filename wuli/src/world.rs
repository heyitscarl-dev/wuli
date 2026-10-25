use glam::{DVec2, dvec2};
use slotmap::SlotMap;

use crate::{
    HandleNotFound,
    body::{BodyHandle, RigidBody},
    collision::{self, narrow::Collision},
    constraints::{self, xpbd::scalar_cross_product},
};

pub const SUBSTEPS: usize = 20; // FIXME: Optimal?

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

    fn collect_possible_collisions(&self, dt: f64) -> Vec<(BodyHandle, BodyHandle)> {
        collision::broad::get_possible_collisions(self, dt)
    }

    fn collect_collisions(&self, possible: &Vec<(BodyHandle, BodyHandle)>) -> Vec<Collision> {
        possible
            .iter()
            .filter_map(|(a, b)| collision::narrow::get_collision(self, *a, *b))
            .collect()
    }

    pub fn step(&mut self, dt: f64) {
        if dt == 0.0 {
            return;
        } // or get div by 0 errors

        let possible_collisions = self.collect_possible_collisions(dt);
        let h = dt / SUBSTEPS as f64;

        for _ in 0..SUBSTEPS {
            self.substep(h, &possible_collisions)
                .expect("expected body to exist");
        }
    }

    pub fn substep(
        &mut self,
        h: f64,
        possible_collisions: &Vec<(BodyHandle, BodyHandle)>,
    ) -> Result<(), HandleNotFound> {
        for (_, b) in &mut self.bodies {
            let force = 0.0; // TODO: Force accumulation
            let torque = 0.0;

            if b.inv_mass == 0.0 {
                continue;
            }

            b.prev_position = b.position;
            b.velocity += (self.gravity + force * b.inv_mass) * h; // Semi-implicit euler integration
            b.position += b.velocity * h;

            // FIXME: Torque/Angular velocity
            b.prev_angle = b.angle;
            b.angular_velocity += (torque) * h * b.inv_inertia; // incorrect?
            b.angle += b.angular_velocity * h;
        }

        let mut collisions = self.collect_collisions(&possible_collisions);
        for collision in &mut collisions {
            constraints::handle_collision(self, collision, h)?;
        }

        for (_, b) in &mut self.bodies {
            b.prev_velocity = b.velocity;
            b.velocity = (b.position - b.prev_position) / h;
            b.prev_angular_velocity = b.angular_velocity;
            b.angular_velocity = (b.angle - b.prev_angle) / h;
        }

        let e = 1.0; // perfectly elastic collisions
        for collision in &mut collisions {
            let [a, b] = self.bodies_mut([collision.a, collision.b])?;
            let vn_before = collision.normal.dot(b.prev_velocity - a.prev_velocity);
            if vn_before >= 0.0 {
                continue;
            } // skip already separating bodies
            let vn_after = collision.normal.dot(b.velocity - a.velocity);
            let v_target = -e * vn_before;
            let dv = v_target - vn_after;
            let r_a = collision.contact - a.position;
            let r_b = collision.contact - b.position;
            let wa = a.inv_mass
                + a.inv_inertia
                    * scalar_cross_product(r_a, collision.normal)
                    * scalar_cross_product(r_a, collision.normal);
            let wb = b.inv_mass
                + b.inv_inertia
                    * scalar_cross_product(r_b, collision.normal)
                    * scalar_cross_product(r_b, collision.normal);
            if wa + wb == 0.0 {
                continue;
            } // skip two static bodies (again)
            let p = dv / (wa + wb) * collision.normal;

            a.velocity -= p * a.inv_mass;
            b.velocity += p * b.inv_mass;
            a.angular_velocity -= a.inv_inertia * scalar_cross_product(r_a, p);
            b.angular_velocity += b.inv_inertia * scalar_cross_product(r_b, p);
        }

        Ok(())
    }

    pub fn handles(&self) -> Vec<BodyHandle> {
        self.bodies.keys().collect()
    }

    pub fn body(&self, handle: BodyHandle) -> Result<&RigidBody, HandleNotFound> {
        self.bodies.get(handle).ok_or(HandleNotFound)
    }

    pub fn body_at(&self, point: DVec2) -> Option<BodyHandle> {
        self.bodies
            .iter()
            .find(|(_, b)| b.shape.contains(b.position, b.angle, point))
            .map(|(h, _)| h)
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
