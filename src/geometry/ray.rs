use crate::geometry::collision;
use crate::geometry::{Aabb, Intersects, Triangle, Vector3};

/// One-sided infinite ray in three-dimensional Cartesian space.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    origin: Vector3,
    direction: Vector3,
}

impl Ray {
    /// Construct a Ray from its origin and direction
    pub fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray { origin, direction }
    }

    /// Get the origin
    pub fn origin(&self) -> Vector3 {
        self.origin
    }

    /// Get the direction
    pub fn direction(&self) -> Vector3 {
        self.direction
    }
}

impl Intersects<Aabb> for Ray {
    fn intersects(&self, aabb: &Aabb) -> bool {
        collision::intersects_aabb_ray(aabb, self)
    }
}

impl Intersects<Triangle> for Ray {
    fn intersects(&self, triangle: &Triangle) -> bool {
        collision::intersects_ray_triangle(self, triangle)
    }
}
