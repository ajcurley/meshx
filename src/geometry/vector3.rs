use pyo3::exceptions::PyIndexError;
use pyo3::prelude::*;

use crate::geometry::collision;
use crate::geometry::{Aabb, Intersects, Sphere};

/// Vector3 in three-dimensional Cartesian space.
#[pyclass]
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

#[pymethods]
impl Vector3 {
    /// Construct a Vector3 from its components
    #[new]
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }

    /// Construct a Vector3 of all zeros
    #[staticmethod]
    pub fn zeros() -> Vector3 {
        Vector3::new(0., 0., 0.)
    }

    /// Construct a Vector3 of all ones
    #[staticmethod]
    pub fn ones() -> Vector3 {
        Vector3::new(1., 1., 1.)
    }

    /// Compute the vector dot product u * v
    #[staticmethod]
    pub fn dot(u: &Vector3, v: &Vector3) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    /// Compute the vector cross product u x v
    #[staticmethod]
    pub fn cross(u: &Vector3, v: &Vector3) -> Vector3 {
        Vector3 {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
    }

    /// Compute the angle (in radians) between u and v
    #[staticmethod]
    pub fn angle(u: &Vector3, v: &Vector3) -> f64 {
        (Vector3::dot(u, v) / (u.mag() * v.mag())).acos()
    }

    /// Get the x-component
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Get the y-component
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Get the z-component
    pub fn z(&self) -> f64 {
        self.z
    }

    /// Compute the magnitude (L2-norm)
    pub fn mag(&self) -> f64 {
        Vector3::dot(self, self).sqrt()
    }

    /// Compute the unit vector
    pub fn unit(&self) -> Vector3 {
        *self / self.mag()
    }

    /// Compute the inverse
    pub fn inv(&self) -> Vector3 {
        Vector3 {
            x: 1. / self.x,
            y: 1. / self.y,
            z: 1. / self.z,
        }
    }

    /// (Python) Get the value at the index
    pub fn __getitem__(&self, index: usize) -> PyResult<f64> {
        if index >= 3 {
            return Err(PyIndexError::new_err("index out of range"));
        }

        Ok(self[index])
    }

    /// (Python) Set the value at the index
    pub fn __setitem__(&mut self, index: usize, value: f64) -> PyResult<()> {
        if index >= 3 {
            return Err(PyIndexError::new_err("index out of range"));
        }

        self[index] = value;

        Ok(())
    }

    /// (Python) Check for equality
    pub fn __eq__(&self, other: Vector3) -> bool {
        *self == other
    }

    /// (Python) Add using the + operator
    pub fn __add__(&self, other: Vector3) -> Vector3 {
        *self + other
    }

    /// (Python) Add using the += operator
    pub fn __iadd__(&mut self, other: Vector3) {
        *self += other;
    }

    /// (Python) Subtract using the - operator
    pub fn __sub__(&self, other: Vector3) -> Vector3 {
        *self - other
    }

    /// (Python) Subtract using the -= operator
    pub fn __isub__(&mut self, other: Vector3) {
        *self -= other;
    }

    /// (Python) Multiply using the * operator
    pub fn __mul__(&self, other: Vector3) -> Vector3 {
        *self * other
    }

    /// (Python) Multiply using the *= operator
    pub fn __imul__(&mut self, other: Vector3) {
        *self *= other;
    }

    /// (Python) Divide using the / operator
    pub fn __truediv__(&self, other: Vector3) -> Vector3 {
        *self / other
    }

    /// (Python) Divide using the /= operator
    pub fn __itruediv__(&mut self, other: Vector3) {
        *self /= other;
    }

    /// (Python) Negate using the - operator
    pub fn __neg__(&self) -> Vector3 {
        -(*self)
    }
}

impl std::ops::Index<usize> for Vector3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of range"),
        }
    }
}

impl std::ops::IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of range"),
        }
    }
}

impl std::ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Self::Output {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Add<f64> for Vector3 {
    type Output = Vector3;

    fn add(self, other: f64) -> Self::Output {
        Vector3 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl std::ops::Add<Vector3> for f64 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Self::Output {
        Vector3 {
            x: self + other.x,
            y: self + other.y,
            z: self + other.z,
        }
    }
}

impl std::ops::AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, other: Vector3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl std::ops::AddAssign<f64> for Vector3 {
    fn add_assign(&mut self, other: f64) {
        self.x += other;
        self.y += other;
        self.z += other;
    }
}

impl std::ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Self::Output {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Sub<f64> for Vector3 {
    type Output = Vector3;

    fn sub(self, other: f64) -> Self::Output {
        Vector3 {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

impl std::ops::Sub<Vector3> for f64 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Self::Output {
        Vector3 {
            x: self - other.x,
            y: self - other.y,
            z: self - other.z,
        }
    }
}

impl std::ops::SubAssign<Vector3> for Vector3 {
    fn sub_assign(&mut self, other: Vector3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl std::ops::SubAssign<f64> for Vector3 {
    fn sub_assign(&mut self, other: f64) {
        self.x -= other;
        self.y -= other;
        self.z -= other;
    }
}

impl std::ops::Mul<Vector3> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Self::Output {
        Vector3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl std::ops::Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: f64) -> Self::Output {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl std::ops::Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Self::Output {
        Vector3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl std::ops::MulAssign<Vector3> for Vector3 {
    fn mul_assign(&mut self, other: Vector3) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl std::ops::MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, other: f64) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl std::ops::Div<Vector3> for Vector3 {
    type Output = Vector3;

    fn div(self, other: Vector3) -> Self::Output {
        Vector3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl std::ops::Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, other: f64) -> Self::Output {
        Vector3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl std::ops::Div<Vector3> for f64 {
    type Output = Vector3;

    fn div(self, other: Vector3) -> Self::Output {
        Vector3 {
            x: self / other.x,
            y: self / other.y,
            z: self / other.z,
        }
    }
}

impl std::ops::DivAssign<Vector3> for Vector3 {
    fn div_assign(&mut self, other: Vector3) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl std::ops::DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, other: f64) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

impl std::ops::Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Intersects<Aabb> for Vector3 {
    fn intersects(&self, aabb: &Aabb) -> bool {
        collision::intersects_aabb_vector3(aabb, self)
    }
}

impl Intersects<Sphere> for Vector3 {
    fn intersects(&self, sphere: &Sphere) -> bool {
        collision::intersects_sphere_vector3(sphere, self)
    }
}
