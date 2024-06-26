pub mod aabb;
pub mod collision;
pub mod line;
pub mod plane;
pub mod polygon;
pub mod ray;
pub mod sphere;
pub mod triangle;
pub mod vector3;

// Re-exports
pub use aabb::Aabb;
pub use collision::{Clip, Distance, Intersection, Intersects};
pub use line::Line;
pub use plane::Plane;
pub use polygon::Polygon;
pub use ray::Ray;
pub use sphere::Sphere;
pub use triangle::Triangle;
pub use vector3::Vector3;

/// Geometric tolerance
pub const EPSILON: f64 = 1.0e-8;
