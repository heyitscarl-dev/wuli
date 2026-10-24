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

    const ENLARGEMENT_MARGIN: f64 = 0.125;
    pub fn enlarged_from_body(body: &RigidBody, dt: f64) -> Self {
        let dimensions = body.bounds();
        // Enlarge by |v| * dt to encompass possible movement
        let delta = body.velocity.length() * dt + Self::ENLARGEMENT_MARGIN;
        Self {
            min: dimensions.0 - delta,
            max: dimensions.1 + delta,
        }
    }
}
