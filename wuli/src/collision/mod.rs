pub mod aabb;
pub mod broad;
pub mod narrow;

trait CouldCollide {
    fn could_collide(&self) -> bool;
}
