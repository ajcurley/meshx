pub mod aabb_aabb;
pub mod aabb_ray;
pub mod aabb_sphere;
pub mod aabb_triangle;
pub mod aabb_vector3;
pub mod line_plane;
pub mod plane_vector3;
pub mod ray_sphere;
pub mod ray_triangle;
pub mod sphere_sphere;
pub mod sphere_triangle;
pub mod sphere_vector3;
pub mod triangle_triangle;
pub mod triangle_vector3;

/// Re-exports
pub use aabb_aabb::intersects_aabb_aabb;
pub use aabb_ray::intersects_aabb_ray;
pub use aabb_sphere::intersects_aabb_sphere;
pub use aabb_triangle::intersects_aabb_triangle;
pub use aabb_vector3::intersects_aabb_vector3;
pub use line_plane::*;
pub use plane_vector3::distance_plane_vector3;
pub use ray_sphere::intersects_ray_sphere;
pub use ray_triangle::intersects_ray_triangle;
pub use sphere_sphere::intersects_sphere_sphere;
pub use sphere_triangle::intersects_sphere_triangle;
pub use sphere_vector3::intersects_sphere_vector3;
pub use triangle_triangle::intersects_triangle_triangle;
pub use triangle_vector3::intersects_triangle_vector3;

/// Check if the two geometries spatially intersect.
pub trait Intersects<T> {
    fn intersects(&self, other: &T) -> bool;
}

/// Compute the intersection geometry.
pub trait Intersection<T> {
    type Output;

    fn intersection(&self, other: &T) -> Option<Self::Output>;
}

/// Compute the minimum distance between two geometries.
pub trait Distance<T> {
    fn distance(&self, other: &T) -> f64;
}

/// Clip a geometry by the cutter geometry.
pub trait Clip<T> {
    type Output;

    fn clip(&self, other: &T) -> Option<Self::Output>;
}
