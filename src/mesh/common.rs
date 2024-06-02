use pyo3::exceptions::PyIndexError;
use pyo3::prelude::*;

use crate::geometry::Vector3;

#[pyclass]
#[derive(Debug, Copy, Clone, Default)]
pub struct Vertex {
    x: f64,
    y: f64,
    z: f64,
}

#[pymethods]
impl Vertex {
    /// Construct a Vertex from its components
    #[new]
    pub fn new(x: f64, y: f64, z: f64) -> Vertex {
        Vertex { x, y, z }
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

    /// Convert to a Vector3
    pub fn to_vector3(&self) -> Vector3 {
        (*self).into()
    }

    /// Convert from a Vector3
    #[staticmethod]
    pub fn from_vector3(value: Vector3) -> Vertex {
        Vertex::from(value)
    }

    /// (Python) Get the component by index
    pub fn __getitem__(&self, index: usize) -> PyResult<f64> {
        if index > 2 {
            return Err(PyIndexError::new_err("index out of range"));
        }

        Ok(self[index])
    }

    /// (Python) Set the component by index
    pub fn __setitem__(&mut self, index: usize, value: f64) -> PyResult<()> {
        if index > 2 {
            return Err(PyIndexError::new_err("index out of range"));
        }

        self[index] = value;

        Ok(())
    }
}

impl std::ops::Index<usize> for Vertex {
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

impl std::ops::IndexMut<usize> for Vertex {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of range"),
        }
    }
}

impl From<Vector3> for Vertex {
    fn from(value: Vector3) -> Vertex {
        Vertex::new(value[0], value[1], value[2])
    }
}

impl Into<Vector3> for Vertex {
    fn into(self) -> Vector3 {
        Vector3::new(self.x, self.y, self.z)
    }
}

#[pyclass]
#[derive(Debug, Clone, Default)]
pub struct Face {
    vertices: Vec<usize>,
    patch: Option<usize>,
}

#[pymethods]
impl Face {
    /// Construct a Face from its vertices and patch
    #[new]
    pub fn new(vertices: Vec<usize>, patch: Option<usize>) -> Face {
        Face { vertices, patch }
    }

    /// Get a borrowed reference to the vertices
    pub fn vertices(&self) -> Vec<usize> {
        self.vertices.clone()
    }

    /// Get the patch
    pub fn patch(&self) -> Option<usize> {
        self.patch
    }

    /// Compute the edges from adjacent vertices
    pub fn edges(&self) -> Vec<Edge> {
        let n = self.vertices.len();
        let mut edges = vec![];

        for (i, &p) in self.vertices.iter().enumerate() {
            let q = self.vertices[(i + 1) % n];
            let edge = Edge::new(p, q, self.patch);
            edges.push(edge);
        }

        edges
    }
}

impl std::ops::Index<usize> for Face {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vertices[index]
    }
}

impl std::ops::IndexMut<usize> for Face {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vertices[index]
    }
}

#[pyclass]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Edge {
    p: usize,
    q: usize,
    patch: Option<usize>,
}

#[pymethods]
impl Edge {
    /// Construct an Edge from its vertices and patch
    #[new]
    pub fn new(p: usize, q: usize, patch: Option<usize>) -> Edge {
        Edge { p, q, patch }
    }

    /// Get the patch
    pub fn patch(&self) -> Option<usize> {
        self.patch
    }

    /// Get the sorted representation
    pub fn sorted(&self) -> Edge {
        Edge {
            p: self.p.min(self.q),
            q: self.p.max(self.q),
            patch: self.patch,
        }
    }
}

impl std::ops::Index<usize> for Edge {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.p,
            1 => &self.q,
            _ => panic!("index out of range"),
        }
    }
}

impl std::ops::IndexMut<usize> for Edge {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.p,
            1 => &mut self.q,
            _ => panic!("index out of range"),
        }
    }
}

impl std::hash::Hash for Edge {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.p.hash(state);
        self.q.hash(state);
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct Patch {
    name: String,
}

#[pymethods]
impl Patch {
    /// Construct a Patch from its name
    #[new]
    pub fn new(name: String) -> Patch {
        Patch { name }
    }

    /// Get a borrowed reference to the name
    pub fn name(&self) -> &str {
        &self.name
    }
}
