use glam::{DVec2, dvec2};
use slotmap::SlotMap;

use crate::{
    HandleNotFound,
    body::{BodyHandle, RigidBody},
    collision::{self, narrow::Collision},
    constraints::{Constraint, ConstraintHandle, ConstraintKind},
};

pub const SUBSTEPS: usize = 20; // FIXME: Optimal?

#[derive(Clone, Default)]
pub struct World {
    pub gravity: DVec2, // e.g. $g =-9.81m/s^2$
    pub bodies: SlotMap<BodyHandle, RigidBody>,
    pub constraints: SlotMap<ConstraintHandle, Constraint>,
}

impl World {
    pub fn new(gravity: f64) -> Self {
        Self {
            gravity: dvec2(0.0, gravity),
            ..Default::default()
        }
    }

    pub fn remove_body(&mut self, handle: BodyHandle) -> Result<RigidBody, HandleNotFound> {
        self.bodies.remove(handle).ok_or(HandleNotFound)
    }

    pub fn remove_constraint(
        &mut self,
        handle: ConstraintHandle,
    ) -> Result<Constraint, HandleNotFound> {
        self.constraints.remove(handle).ok_or(HandleNotFound)
    }

    pub fn clean_remove_body(&mut self, handle: BodyHandle) -> Result<RigidBody, HandleNotFound> {
        let body = self.bodies.remove(handle).ok_or(HandleNotFound)?;

        // Remove all constraints that reference this body.
        let constraints_to_remove: Vec<ConstraintHandle> = self
            .constraints
            .iter()
            .filter(|(_, constraint)| constraint.kind.bodies().contains(&handle))
            .map(|(handle, _)| handle)
            .collect();

        for constraint_handle in constraints_to_remove {
            self.constraints.remove(constraint_handle);
        }

        Ok(body)
    }

    pub fn add_body(&mut self, body: RigidBody) -> BodyHandle {
        self.bodies.insert(body)
    }

    pub fn add_constraint(&mut self, constraint: Constraint) -> ConstraintHandle {
        self.constraints.insert(constraint)
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

    /// Integrate forces, accelerations and velocities to update the positions of all bodies.
    fn integrate_positions(&mut self, h: f64) {
        let bodies: Vec<BodyHandle> = self
            .bodies
            .iter_mut()
            .filter(|(_, body)| !body.is_static())
            .map(|(handle, _)| handle)
            .collect();

        bodies.iter().for_each(|body| {
            let _ = self.integrate_position(*body, h); // ignore failed integrations
        });
    }

    /// Integrate forces, accelerations and velocities to update the position of a single body.
    fn integrate_position(&mut self, body: BodyHandle, h: f64) -> Result<(), HandleNotFound> {
        let gravity = self.gravity;
        let body = self.body_mut(body)?;

        let force = DVec2::ZERO; // TODO: force accumulation
        let torque = 0.0;

        debug_assert!(body.inv_mass != 0.0, "Cannot integrate static body!"); // static bodies should be filtered.

        // The next steps of XPBD update positions, without updating velocities.
        // We set `prev_position` and `prev_angle` to deduce velocities later.
        body.prev_position = body.position;
        body.prev_angle = body.angle;

        // Semi-Implicit Euler
        body.velocity += force * body.inv_mass * h + gravity * h;
        body.position += body.velocity * h;

        body.angular_velocity += torque * body.inv_inertia * h;
        body.angle += body.angular_velocity * h;

        Ok(())
    }

    /// Accumulate "real" constraints (distance, spring, etc.; predefined) with "pseudo" constraints,
    /// derived from collisions.
    fn accumulate_constraints(
        &self,
        possible_collisions: &Vec<(BodyHandle, BodyHandle)>,
    ) -> Vec<Constraint> {
        // XPBD can handle collisions as simple constraints. This merges the processes of collision and
        // constraint solving into one, simplifying the pipeline.
        let constraints = self.constraints.clone();
        let pseudo_constraints = self
            .collect_collisions(possible_collisions)
            .into_iter()
            .map(|collision| Constraint {
                kind: ConstraintKind::Contact { collision },
                soft: false,
            });

        constraints
            .iter()
            .map(|(_, constraint)| constraint.clone())
            .chain(pseudo_constraints)
            .collect()
    }

    pub fn energy(&self) -> f64 {
        self.bodies
            .iter()
            .map(|(_, body)| body.kinetic_energy() + body.potential_energy(self.gravity))
            .sum()
    }

    pub fn substep(
        &mut self,
        h: f64,
        possible_collisions: &Vec<(BodyHandle, BodyHandle)>,
    ) -> Result<Vec<Constraint>, HandleNotFound> {
        self.integrate_positions(h);
        let mut constraints = self.accumulate_constraints(possible_collisions);

        const ITERATIONS: usize = 4;
        for _ in 0..ITERATIONS {
            // solve soft constraints first, so they get overriden later
            constraints
                .iter_mut()
                .filter(|constraint| constraint.soft)
                .for_each(|constraint| constraint.kind.solve_positions(self, h));

            constraints
                .iter_mut()
                .filter(|constraint| !constraint.soft)
                .for_each(|constraint| constraint.kind.solve_positions(self, h));
        }

        // let mut collisions = self.collect_collisions(&possible_collisions);
        // for collision in &mut collisions {
        //    constraints::handle_collision(self, collision, h)?;
        // }

        for (_, b) in &mut self.bodies {
            b.prev_velocity = b.velocity;
            b.velocity = (b.position - b.prev_position) / h;
            b.prev_angular_velocity = b.angular_velocity;
            b.angular_velocity = (b.angle - b.prev_angle) / h;
        }

        constraints
            .iter_mut()
            .for_each(|constraint| constraint.kind.solve_velocities(self, h));

        Ok(constraints)
    }

    pub fn handles(&self) -> Vec<BodyHandle> {
        self.bodies.keys().collect()
    }

    pub fn body(&self, handle: BodyHandle) -> Result<&RigidBody, HandleNotFound> {
        self.bodies.get(handle).ok_or(HandleNotFound)
    }

    pub fn constraint(&self, handle: ConstraintHandle) -> Result<&Constraint, HandleNotFound> {
        self.constraints.get(handle).ok_or(HandleNotFound)
    }

    pub fn constraint_mut(
        &mut self,
        handle: ConstraintHandle,
    ) -> Result<&mut Constraint, HandleNotFound> {
        self.constraints.get_mut(handle).ok_or(HandleNotFound)
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
