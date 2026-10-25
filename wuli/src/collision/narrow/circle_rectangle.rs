use glam::{DVec2, dvec2};

use crate::{HandleNotFound, World, body::BodyHandle, collision::narrow::Collision};

pub fn get_circle_rectangle_collision(
    world: &World,
    body_circle: BodyHandle,
    body_rectangle: BodyHandle,
    radius: f64,
    half_extents: DVec2,
) -> Result<Option<Collision>, HandleNotFound> {
    let circle = world.body(body_circle)?;
    let rectangle = world.body(body_rectangle)?;

    let local = DVec2::from_angle(-rectangle.angle).rotate(circle.position - rectangle.position);
    let closest = local.clamp(-half_extents, half_extents);
    let delta = local - closest;

    if delta.length() > radius {
        return Ok(None);
    }
    let depth;

    // circle's centre inside the rectangle
    let local_normal = if delta == DVec2::ZERO {
        let gap = half_extents - local.abs();
        let face_normal = if gap.x < gap.y {
            dvec2(local.x.signum(), 0.0)
        } else {
            dvec2(0.0, local.y.signum())
        };
        let distance = gap.x.min(gap.y);
        depth = radius + distance;
        -face_normal
    } else {
        depth = radius - delta.length();
        -delta.normalize()
    };

    // convert back to world space
    let normal = DVec2::from_angle(rectangle.angle).rotate(local_normal);
    let contact = circle.position + normal * radius; // on a's surface, like circle-circle

    Ok(Some(Collision {
        normal,
        depth,
        contact,
        a: body_circle,
        b: body_rectangle,
        lambda: 0.0,
    }))
}
