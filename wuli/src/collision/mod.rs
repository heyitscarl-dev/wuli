pub mod aabb;
pub mod broad;

trait Collides {
    fn collides(&self) -> bool;
}