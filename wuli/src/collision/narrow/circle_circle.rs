use glam::DVec2;

use crate::{World, body::BodyHandle, collision::narrow::Collision};

pub fn get_circle_circle_collision(world: &World, body_a: BodyHandle, body_b: BodyHandle, ra: f64, rb: f64) -> Option<Collision> {
    let a = world.body(body_a).position;
    let b = world.body(body_b).position;
    if (ra + rb) < (a - b).length() { return None; }
    let normal = (b - a).try_normalize().unwrap_or(DVec2::Y); // concentric: any direction separates them
    let depth = ra + rb - (a - b).length();
    let contact =  a + normal * ra;
    Some(Collision { a: body_a, b: body_b, normal, depth, contact })
}