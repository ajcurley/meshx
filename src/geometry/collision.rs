pub mod aabb_aabb;
pub mod aabb_ray;
pub mod aabb_sphere;
pub mod aabb_vector3;
pub mod ray_sphere;
pub mod ray_triangle;
pub mod sphere_sphere;
pub mod sphere_vector3;

/// Re-exports
pub use aabb_aabb::intersects_aabb_aabb;
pub use aabb_ray::intersects_aabb_ray;
pub use aabb_sphere::intersects_aabb_sphere;
pub use aabb_vector3::intersects_aabb_vector3;
pub use ray_sphere::intersects_ray_sphere;
pub use ray_triangle::intersects_ray_triangle;
pub use sphere_sphere::intersects_sphere_sphere;
pub use sphere_vector3::intersects_sphere_vector3;

/// Check if the two geometries spatiall intersect.
pub trait Intersects<T> {
    fn intersects(&self, other: &T) -> bool;
}
