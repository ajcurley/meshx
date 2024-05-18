use pyo3::exceptions::PyIndexError;
use pyo3::prelude::*;

use crate::geometry::collision;
use crate::geometry::{Aabb, Intersects, Ray, Vector3};

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

impl Intersects<Ray> for Triangle {
    fn intersects(&self, ray: &Ray) -> bool {
        collision::intersects_ray_triangle(ray, self)
    }
}
