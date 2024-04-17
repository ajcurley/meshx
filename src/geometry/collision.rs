pub mod aabb_aabb;
pub mod aabb_ray;
pub mod aabb_vector3;
pub mod ray_triangle;

/// Re-exports
pub use aabb_aabb::intersects_aabb_aabb;
pub use aabb_ray::intersects_aabb_ray;
pub use aabb_vector3::intersects_aabb_vector3;
pub use ray_triangle::intersects_ray_triangle;

/// Check if the two geometries spatiall intersect.
pub trait Intersects<T> {
    fn intersects(&self, other: &T) -> bool;
}
