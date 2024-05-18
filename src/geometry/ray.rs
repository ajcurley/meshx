use pyo3::prelude::*;

use crate::geometry::collision;
use crate::geometry::{Aabb, Intersects, Sphere, Triangle, Vector3};

/// One-sided infinite ray in three-dimensional Cartesian space.
#[pyclass]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    origin: Vector3,
    direction: Vector3,
}

#[pymethods]
impl Ray {
    /// Construct a Ray from its origin and direction
    #[new]
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

impl Intersects<Sphere> for Ray {
    fn intersects(&self, sphere: &Sphere) -> bool {
        collision::intersects_ray_sphere(self, sphere)
    }
}

impl Intersects<Triangle> for Ray {
    fn intersects(&self, triangle: &Triangle) -> bool {
        collision::intersects_ray_triangle(self, triangle)
    }
}
