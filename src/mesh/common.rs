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
    #[getter]
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Set the x-component
    pub fn set_x(&mut self, value: f64) {
        self.x = value;
    }

    /// Get the y-component
    #[getter]
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Set the y-component
    pub fn set_y(&mut self, value: f64) {
        self.y = value;
    }

    /// Get the z-component
    #[getter]
    pub fn z(&self) -> f64 {
        self.z
    }

    /// Set the z-component
    pub fn set_z(&mut self, value: f64) {
        self.z = value;
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

    /// Get the vertices
    #[getter]
    pub fn vertices(&self) -> Vec<usize> {
        self.vertices.clone()
    }

    /// Set the vertices
    #[setter]
    pub fn set_vertices(&mut self, vertices: Vec<usize>) {
        self.vertices = vertices;
    }

    /// Get the patch
    #[getter]
    pub fn patch(&self) -> Option<usize> {
        self.patch
    }

    /// Set the patch
    #[setter]
    pub fn set_patch(&mut self, patch: Option<usize>) {
        self.patch = patch;
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

    /// (Python) Get a vertex index by index
    pub fn __getitem__(&self, index: usize) -> PyResult<usize> {
        if index >= self.vertices.len() {
            return Err(PyIndexError::new_err("index out of range"));
        }

        Ok(self.vertices[index])
    }

    /// (Python) Set a vertex index by index
    pub fn __setitem__(&mut self, index: usize, value: usize) -> PyResult<()> {
        if index >= self.vertices.len() {
            return Err(PyIndexError::new_err("index out of range"));
        }

        self.vertices[index] = value;
        Ok(())
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
#[derive(Debug, Copy, Clone)]
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

    /// Get the p-component
    #[getter]
    pub fn p(&self) -> usize {
        self.p
    }

    /// Set the p-component
    #[setter]
    pub fn set_p(&mut self, value: usize) {
        self.p = value;
    }

    /// Get the q-component
    #[getter]
    pub fn q(&self) -> usize {
        self.q
    }

    /// Set the q-component
    pub fn set_q(&mut self, value: usize) {
        self.q = value;
    }

    /// Get the patch
    #[getter]
    pub fn patch(&self) -> Option<usize> {
        self.patch
    }

    /// Set the patch
    #[setter]
    pub fn set_patch(&mut self, patch: Option<usize>) {
        self.patch = patch;
    }

    /// Get the sorted representation
    pub fn sorted(&self) -> Edge {
        Edge {
            p: self.p.min(self.q),
            q: self.p.max(self.q),
            patch: self.patch,
        }
    }

    /// (Python) Get the vertex index by index
    pub fn __getitem__(&self, index: usize) -> PyResult<usize> {
        if index > 1 {
            return Err(PyIndexError::new_err("index out of range"));
        }

        Ok(self[index])
    }

    /// (Python) Set the vertex index by index
    pub fn __setitem__(&mut self, index: usize, value: usize) -> PyResult<()> {
        if index > 1 {
            return Err(PyIndexError::new_err("index out of range"));
        }

        self[index] = value;
        Ok(())
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

impl std::cmp::PartialEq for Edge {
    fn eq(&self, other: &Edge) -> bool {
        self.p == other.p && self.q == other.q
    }
}

impl std::cmp::Eq for Edge {}

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
    #[getter]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the name
    #[setter]
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}
