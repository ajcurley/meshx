use crate::geometry::collision;
use crate::geometry::{Intersects, Ray, Sphere, Vector3};

/// Axis-aligned bounding box in three-dimensional Cartesian space.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Aabb {
    center: Vector3,
    halfsize: Vector3,
}

impl Aabb {
    /// Construct an Aabb from its center and halfsize
    pub fn new(center: Vector3, halfsize: Vector3) -> Aabb {
        Aabb { center, halfsize }
    }

    /// Construct an Aabb from its min and max bounds
    pub fn from_bounds(min: Vector3, max: Vector3) -> Aabb {
        let center = (max + min) * 0.5;
        let halfsize = (max - min) * 0.5;
        Aabb::new(center, halfsize)
    }

    /// Get the center
    pub fn center(&self) -> Vector3 {
        self.center
    }

    /// Get the halfsize
    pub fn halfsize(&self) -> Vector3 {
        self.halfsize
    }

    /// Compute the min bound
    pub fn min(&self) -> Vector3 {
        self.center - self.halfsize
    }

    /// Compute the max boun
    pub fn max(&self) -> Vector3 {
        self.center + self.halfsize
    }
}

impl Intersects<Aabb> for Aabb {
    fn intersects(&self, aabb: &Aabb) -> bool {
        collision::intersects_aabb_aabb(self, aabb)
    }
}

impl Intersects<Ray> for Aabb {
    fn intersects(&self, ray: &Ray) -> bool {
        collision::intersects_aabb_ray(self, ray)
    }
}

impl Intersects<Sphere> for Aabb {
    fn intersects(&self, sphere: &Sphere) -> bool {
        collision::intersects_aabb_sphere(self, sphere)
    }
}

impl Intersects<Vector3> for Aabb {
    fn intersects(&self, v: &Vector3) -> bool {
        collision::intersects_aabb_vector3(self, v)
    }
}
