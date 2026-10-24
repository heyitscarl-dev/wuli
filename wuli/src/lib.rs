pub mod body;
pub mod collision;
pub mod world;

pub use body::RigidBody;
use thiserror::Error;
pub use world::World;

#[derive(Debug, Error)]
#[error("handle not found")]
pub struct HandleNotFound;
