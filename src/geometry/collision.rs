pub mod aabb_vector3;

/// Re-exports
pub use aabb_vector3::intersects_aabb_vector3;

/// Check if the two geometries spatiall intersect.
pub trait Intersects<T> {
    fn intersects(&self, other: &T) -> bool;
}
