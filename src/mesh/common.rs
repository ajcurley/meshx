use crate::geometry::Vector3;

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Vertex {
    x: f64,
    y: f64,
    z: f64,
}

impl Vertex {
    /// Construct a Vertex from its components
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

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Face {
    vertices: Vec<usize>,
    patch: Option<usize>,
}

impl Face {
    /// Construct a Face from its vertices and patch
    pub fn new(vertices: Vec<usize>, patch: Option<usize>) -> Face {
        Face { vertices, patch }
    }

    /// Get a borrowed reference to the vertices
    pub fn vertices(&self) -> &Vec<usize> {
        &self.vertices
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Edge {
    p: usize,
    q: usize,
    patch: Option<usize>,
}

impl Edge {
    /// Construct an Edge from its vertices and patch
    pub fn new(p: usize, q: usize, patch: Option<usize>) -> Edge {
        Edge { p, q, patch }
    }

    /// Get the p-component
    pub fn p(&self) -> usize {
        self.p
    }

    /// Get the q-component
    pub fn q(&self) -> usize {
        self.q
    }

    /// Get the patch
    pub fn patch(&self) -> Option<usize> {
        self.patch
    }

    /// Convert the Edge to a sorted tuple
    pub fn as_tuple(&self) -> (usize, usize) {
        if self.p <= self.q {
            return (self.p, self.q);
        }
        (self.q, self.p)
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

#[derive(Debug, Clone, PartialEq)]
pub struct Patch {
    name: String,
}

impl Patch {
    /// Construct a Patch from its name
    pub fn new(name: String) -> Patch {
        Patch { name }
    }

    /// Get a borrowed reference to the name
    pub fn name(&self) -> &str {
        &self.name
    }
}
