use glam::DVec2;

use crate::{World, collision::narrow::Collision};

pub fn handle_collision(world: &mut World, collision: Collision) {
    let impulse;
    let total_mass;
    {
        let a = world.body(collision.a);
        let b = world.body(collision.b);
        let normal_velocity = DVec2::dot(b.velocity - a.velocity, collision.normal);
        if normal_velocity > 0.0 {
            return;
        } // already seperating naturally
        let restitution = 1.0; // elastic
        impulse = -(1.0 + restitution) * normal_velocity / (a.inv_mass + b.inv_mass);
        total_mass = a.inv_mass + b.inv_mass;
    }
    {
        let a = world.body_mut(collision.a);
        a.velocity -= impulse * collision.normal * a.inv_mass;
        a.position -= collision.depth * collision.normal * (a.inv_mass / total_mass);
    }
    let b = world.body_mut(collision.b);
    b.velocity += impulse * collision.normal * b.inv_mass;
    b.position += collision.depth * collision.normal * (b.inv_mass / total_mass);
}
