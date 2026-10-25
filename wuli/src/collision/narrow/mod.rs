use glam::DVec2;

use crate::{
    World, body::{BodyHandle, Shape}, collision::narrow::{circle_circle::get_circle_circle_collision, circle_rectangle::get_circle_rectangle_collision},
};

pub mod circle_circle;
pub mod circle_rectangle;

pub struct Collision {
    pub normal: DVec2,
    pub depth: f64,
    pub contact: DVec2,
    pub a: BodyHandle,
    pub b: BodyHandle,
    pub lambda: f64,
}

pub fn get_collision(world: &World, body_a: BodyHandle, body_b: BodyHandle) -> Option<Collision> {
    let a = world.body(body_a).expect("expected body to exist");
    let b = world.body(body_b).expect("expected body to exits");
    match (&a.shape, &b.shape) {
        (Shape::Circle { radius: ra }, Shape::Circle { radius: rb }) => {
            get_circle_circle_collision(world, body_a, body_b, *ra, *rb)
        },
        (Shape::Circle { radius }, Shape::Rectangle { half_extents }) => {
            get_circle_rectangle_collision(world, body_a, body_b, *radius, *half_extents)
                .ok().flatten() // ignore error values, since deleted handles can't collide
        },
        (Shape::Rectangle { half_extents }, Shape::Circle { radius }) => {
            let collision = get_circle_rectangle_collision(
                world, body_b, body_a, *radius, *half_extents
            ).ok().flatten();
            collision.and_then(|c| Some(Collision {
                a: body_a,
                b: body_b,
                normal: -c.normal,
                depth: c.depth,
                contact: c.contact,
                lambda: c.lambda
            }))
        }
        _ => None,
    }
}
