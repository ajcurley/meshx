use crate::geometry::Vector3;

#[derive(Debug, Clone)]
pub struct Polygon {
    vertices: Vec<Vector3>,
}

impl Polygon {
    /// Construct a Polygon from its ordered set of vertices
    pub fn new(vertices: Vec<Vector3>) -> Polygon {
        Polygon { vertices }
    }

    /// Get a borrowed reference to the vertices
    pub fn vertices(&self) -> &Vec<Vector3> {
        &self.vertices
    }
}
