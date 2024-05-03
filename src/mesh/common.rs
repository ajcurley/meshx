#[derive(Debug, Copy, Clone, Default)]
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

#[derive(Debug, Clone, Default)]
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

#[derive(Debug, Copy, Clone)]
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

    /// Get the patch
    pub fn patch(&self) -> Option<usize> {
        self.patch
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

#[derive(Debug, Clone)]
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
