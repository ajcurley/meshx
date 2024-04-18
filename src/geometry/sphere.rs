use crate::geometry::collision;
use crate::geometry::{Aabb, Intersects, Ray, Vector3};

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

impl Intersects<Aabb> for Sphere {
    fn intersects(&self, aabb: &Aabb) -> bool {
        collision::intersects_aabb_sphere(aabb, self)
    }
}

impl Intersects<Ray> for Sphere {
    fn intersects(&self, ray: &Ray) -> bool {
        collision::intersects_ray_sphere(ray, self)
    }
}

impl Intersects<Sphere> for Sphere {
    fn intersects(&self, sphere: &Sphere) -> bool {
        collision::intersects_sphere_sphere(self, sphere)
    }
}

impl Intersects<Vector3> for Sphere {
    fn intersects(&self, v: &Vector3) -> bool {
        collision::intersects_sphere_vector3(self, v)
    }
}
