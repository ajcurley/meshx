use pyo3::exceptions::PyIndexError;
use pyo3::prelude::*;

use crate::geometry::collision;
use crate::geometry::{Aabb, Intersects, Ray, Sphere, Vector3};

/// Triangle in three-dimensional Cartesian space
#[pyclass]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Triangle {
    p: Vector3,
    q: Vector3,
    r: Vector3,
}

#[pymethods]
impl Triangle {
    /// Construct a Triangle from its vertices p, q, and r
    #[new]
    pub fn new(p: Vector3, q: Vector3, r: Vector3) -> Triangle {
        Triangle { p, q, r }
    }

    /// Get the p-vertex
    pub fn p(&self) -> Vector3 {
        self.p
    }

    /// Get the q-vertex
    pub fn q(&self) -> Vector3 {
        self.q
    }

    /// Get the r-vertex
    pub fn r(&self) -> Vector3 {
        self.r
    }

    /// Compute the axis-aligned bounding box
    pub fn aabb(&self) -> Aabb {
        let mut min = Vector3::zeros();
        let mut max = Vector3::zeros();

        for i in 0..3 {
            min[i] = self.p[i].min(self.q[i]).min(self.r[i]);
            max[i] = self.p[i].max(self.q[i]).max(self.r[i]);
        }

        Aabb::from_bounds(min, max)
    }

    /// Compute the area
    pub fn area(&self) -> f64 {
        self.normal().mag() * 0.5
    }

    /// Compute the normal vector (non-normalized)
    pub fn normal(&self) -> Vector3 {
        let u = self.q - self.p;
        let v = self.r - self.p;
        Vector3::cross(&u, &v)
    }

    /// Compute the unit normal vector
    pub fn unit_normal(&self) -> Vector3 {
        self.normal().unit()
    }

    /// Compute the centroid.
    pub fn centroid(&self) -> Vector3 {
        (self.p + self.q + self.r) / 3.
    }

    /// Compute the Barycentric coordinate (u, v, w).
    pub fn barycenter(&self) -> Vector3 {
        let v0 = self.q - self.p;
        let v1 = self.r - self.q;
        let v2 = self.p - self.r;

        let d00 = Vector3::dot(&v0, &v0);
        let d01 = Vector3::dot(&v0, &v1);
        let d11 = Vector3::dot(&v1, &v1);
        let d20 = Vector3::dot(&v2, &v0);
        let d21 = Vector3::dot(&v2, &v1);

        let d = d00 * d11 - d01 * d01;
        let v = (d11 * d20 - d01 * d21) / d;
        let w = (d00 * d21 - d00 * d20) / d;
        let u = 1. - v - w;

        Vector3::new(u, v, w)
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

    /// Check for a spatial intersection with a Triangle
    pub fn intersects_triangle(&self, triangle: &Triangle) -> bool {
        self.intersects(triangle)
    }

    /// (Python) Get a vertex by index
    pub fn __getitem__(&self, index: usize) -> PyResult<Vector3> {
        if index >= 3 {
            return Err(PyIndexError::new_err("index out of range"));
        }

        Ok(self[index])
    }

    /// (Python) Set a vertex by index
    pub fn __setitem__(&mut self, index: usize, value: Vector3) -> PyResult<()> {
        if index >= 3 {
            return Err(PyIndexError::new_err("index out of range"));
        }

        self[index] = value;

        Ok(())
    }
}

impl std::ops::Index<usize> for Triangle {
    type Output = Vector3;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.p,
            1 => &self.q,
            2 => &self.r,
            _ => panic!("index out of range"),
        }
    }
}

impl std::ops::IndexMut<usize> for Triangle {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.p,
            1 => &mut self.q,
            2 => &mut self.r,
            _ => panic!("index out of range"),
        }
    }
}

impl Intersects<Aabb> for Triangle {
    fn intersects(&self, aabb: &Aabb) -> bool {
        collision::intersects_aabb_triangle(aabb, self)
    }
}

impl Intersects<Ray> for Triangle {
    fn intersects(&self, ray: &Ray) -> bool {
        collision::intersects_ray_triangle(ray, self)
    }
}

impl Intersects<Sphere> for Triangle {
    fn intersects(&self, sphere: &Sphere) -> bool {
        collision::intersects_sphere_triangle(sphere, self)
    }
}

impl Intersects<Triangle> for Triangle {
    fn intersects(&self, triangle: &Triangle) -> bool {
        collision::intersects_triangle_triangle(self, triangle)
    }
}

impl Intersects<Vector3> for Triangle {
    fn intersects(&self, v: &Vector3) -> bool {
        collision::intersects_triangle_vector3(self, v)
    }
}
