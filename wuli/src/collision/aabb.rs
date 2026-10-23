use glam::DVec2;

use crate::RigidBody;

pub struct AxisAlignedBoundingBox {
    pub min: DVec2,
    pub max: DVec2,
}

impl AxisAlignedBoundingBox {
    pub fn from_body(body: &RigidBody) -> Self {
        let dimensions = body.bounds();
        Self {
            min: dimensions.0,
            max: dimensions.1,
        }
    }
}
