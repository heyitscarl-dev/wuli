use glam::DVec2;

use crate::{World, body::{BodyHandle, Shape}, collision::narrow::circle_circle::get_circle_circle_collision};

pub mod circle_circle;

pub struct Collision {
    pub normal: DVec2,
    pub depth: f64,
    pub contact: DVec2,
    pub a: BodyHandle,
    pub b: BodyHandle
}

pub fn get_collision(world: &World, body_a: BodyHandle, body_b: BodyHandle) -> Option<Collision> {
    let a = world.body(body_a);
    let b = world.body(body_b);
    match (&a.shape, &b.shape) {
        (Shape::Circle { radius: ra }, Shape::Circle { radius: rb }) 
            => get_circle_circle_collision(world, body_a, body_b, *ra, *rb),
        _ => None
    }
}