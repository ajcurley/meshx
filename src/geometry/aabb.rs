use pyo3::prelude::*;

use crate::geometry::collision;
use crate::geometry::{Intersects, Plane, Ray, Sphere, Vector3};

/// Axis-aligned bounding box in three-dimensional Cartesian space.
#[pyclass]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Aabb {
    center: Vector3,
    halfsize: Vector3,
}

#[pymethods]
impl Aabb {
    /// Construct an Aabb from its center and halfsize
    #[new]
    pub fn new(center: Vector3, halfsize: Vector3) -> Aabb {
        Aabb { center, halfsize }
    }

    /// Construct an Aabb from its min and max bounds
    #[staticmethod]
    pub fn from_bounds(min: Vector3, max: Vector3) -> Aabb {
        let center = (max + min) * 0.5;
        let halfsize = (max - min) * 0.5;
        Aabb::new(center, halfsize)
    }

    /// Construct a unit Aabb
    #[staticmethod]
    pub fn unit() -> Aabb {
        let center = Vector3::zeros();
        let halfsize = Vector3::new(0.5, 0.5, 0.5);
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

    /// Compute the octant axis-aligned bounding box
    pub fn octant(&self, octant: usize) -> Aabb {
        let h = self.halfsize() * 0.5;

        let dx = if (octant & 4) == 0 { -h[0] } else { h[0] };
        let dy = if (octant & 2) == 0 { -h[1] } else { h[1] };
        let dz = if (octant & 1) == 0 { -h[2] } else { h[2] };
        let center = self.center + Vector3::new(dx, dy, dz);

        Aabb::new(center, h)
    }

    /// Get the inward-facing Planes defining the boundary
    pub fn planes(&self) -> Vec<Plane> {
        let min = self.min();
        let max = self.max();
        let mut planes = vec![];

        for i in 0..3 {
            let mut normal = Vector3::zeros();
            normal[i] = 1.;
            let plane = Plane::new(normal, -min[i]);
            planes.push(plane);

            let mut normal = Vector3::zeros();
            normal[i] = -1.;
            let plane = Plane::new(normal, max[i]);
            planes.push(plane);
        }

        planes
    }

    /// Check for a spatial intersection with an Aabb
    pub fn intersects_aabb(&self, aabb: &Aabb) -> bool {
        self.intersects(aabb)
    }

    /// Check for a spatial intersection with a Ray
    pub fn intersects_ray(&self, ray: &Ray) -> bool {
        self.intersects(ray)
    }

    /// Check for a spatial intersection with a Sphere
    pub fn intersects_sphere(&self, sphere: &Sphere) -> bool {
        self.intersects(sphere)
    }

    /// Check for a spatial intersection with a Vector3
    pub fn intersects_vector3(&self, point: &Vector3) -> bool {
        self.intersects(point)
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
    fn intersects(&self, point: &Vector3) -> bool {
        collision::intersects_aabb_vector3(self, point)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_aabb_planes() {
        use crate::geometry::Distance;

        let planes = Aabb::unit().planes();

        assert_eq!(planes.len(), 6);
        assert_eq!(planes[0].distance(&Vector3::new(-0.5, 0., 0.)), 0.);
        assert_eq!(planes[1].distance(&Vector3::new(0.5, 0., 0.)), 0.);
        assert_eq!(planes[2].distance(&Vector3::new(0., -0.5, 0.)), 0.);
        assert_eq!(planes[3].distance(&Vector3::new(0., 0.5, 0.)), 0.);
        assert_eq!(planes[4].distance(&Vector3::new(0., 0., -0.5)), 0.);
        assert_eq!(planes[5].distance(&Vector3::new(0., 0., 0.5)), 0.);
    }
}
