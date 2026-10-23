pub mod aabb;
pub mod broad;
pub mod narrow;
pub mod solve;

trait CouldCollide {
    fn could_collide(&self) -> bool;
}
