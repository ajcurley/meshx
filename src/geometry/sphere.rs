use crate::geometry::{Aabb, Vector3};

/// Sphere in three-dimensional Cartesian space.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere {
    center: Vector3,
    radius: f64,
}

impl Sphere {
    /// Construct a Sphere from its center and radius
    pub fn new(center: Vector3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }

    /// Compute the axis-aligned bounding box
    pub fn aabb(&self) -> Aabb {
        let halfsize = Vector3::ones() * self.radius;
        Aabb::new(self.center, halfsize)
    }

    /// Get the center
    pub fn center(&self) -> Vector3 {
        self.center
    }

    /// Get the radius
    pub fn radius(&self) -> f64 {
        self.radius
    }
}
