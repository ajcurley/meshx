use std::collections::HashMap;

use crate::geometry::Vector3;
use crate::mesh::wavefront::ObjReader;
use crate::mesh::{Edge, Face, Patch, Vertex};

#[derive(Debug, Clone, Default)]
pub struct HeMesh {
    vertices: Vec<HeVertex>,
    faces: Vec<HeFace>,
    half_edges: Vec<HeHalfEdge>,
    patches: Vec<HePatch>,
}

impl HeMesh {
    /// Construct a HeMesh from its components
    pub fn new(vertices: &Vec<Vertex>, faces: &Vec<Face>, patches: &Vec<Patch>) -> HeMesh {
        let mut mesh = HeMesh::default();
        let mut half_edges: HashMap<Edge, Vec<usize>> = HashMap::new();

        // Index the patches
        for patch in patches.iter() {
            let patch = HePatch::from(patch);
            mesh.patches.push(patch);
        }

        // Index the vertices without reference to their originating half
        // edge. The referenced half edges are indexed later.
        for vertex in vertices.iter() {
            let vertex = HeVertex::from(vertex);
            mesh.vertices.push(vertex);
        }

        // Index the faces and their edges (converted to half edge). For each
        // face, use the first half edge as the referenced half edge.
        for (face_id, face) in faces.iter().enumerate() {
            let count = mesh.half_edges.len();
            let n = face.vertices().len();

            for (edge_id, edge) in face.edges().iter().enumerate() {
                let half_edge_id = count + edge_id;
                let mut half_edge = HeHalfEdge::default();
                half_edge.origin = edge[0];
                half_edge.face = face_id;

                if edge_id == 0 {
                    half_edge.prev = count + n - 1;
                    half_edge.next = count + edge_id + 1;
                } else {
                    half_edge.prev = count + edge_id - 1;
                    half_edge.next = count;
                }

                // Insert the half edge and update the originating half edge
                // for the origin (vertex).
                mesh.half_edges.push(half_edge);
                mesh.vertices[half_edge.origin].half_edge = half_edge_id;

                // Index the sorted half edge pair to use when updating the
                // half edge twins and checking for non-manifold edges.
                half_edges
                    .entry(edge.sorted())
                    .and_modify(|h| h.push(half_edge_id))
                    .or_insert(vec![half_edge_id]);
            }

            let patch = face.patch();
            let face = HeFace::new(count, patch);
            mesh.faces.push(face);
        }

        // Index the twin half edge for each non-boundary half edge if and
        // only if the mesh is manifold.
        for (_, shared) in half_edges.iter() {
            if shared.len() > 2 {
                panic!("non-manifold mesh");
            }

            if shared.len() == 2 {
                mesh.half_edges[shared[0]].twin = Some(shared[1]);
                mesh.half_edges[shared[1]].twin = Some(shared[0]);
            }
        }

        mesh
    }

    /// Import a HeMesh from an OBJ file
    pub fn from_obj(filename: &str) -> std::io::Result<HeMesh> {
        let mut reader = ObjReader::new(filename);
        reader.read()?;

        let vertices = reader.vertices();
        let faces = reader.faces();
        let patches = reader.patches();
        let mesh = HeMesh::new(vertices, faces, patches);

        Ok(mesh)
    }

    /// Get a borrowed reference to the vertices
    pub fn vertices(&self) -> &Vec<HeVertex> {
        &self.vertices
    }

    /// Get a borroed reference to a vertex by index
    pub fn vertex(&self, index: usize) -> &HeVertex {
        &self.vertices[index]
    }

    /// Get the number of vertices
    pub fn n_vertices(&self) -> usize {
        self.vertices.len()
    }

    /// Get a borrowed reference to the faces
    pub fn faces(&self) -> &Vec<HeFace> {
        &self.faces
    }

    /// Get a borrowed reference to a face by index
    pub fn face(&self, index: usize) -> &HeFace {
        &self.faces[index]
    }

    /// Get the number of faces
    pub fn n_faces(&self) -> usize {
        self.faces.len()
    }

    /// Get a borrowed reference to the half edges
    pub fn half_edges(&self) -> &Vec<HeHalfEdge> {
        &self.half_edges
    }

    /// Get a borrowed reference to a half edge by index
    pub fn half_edge(&self, index: usize) -> &HeHalfEdge {
        &self.half_edges[index]
    }

    /// Get the number of half edges
    pub fn n_half_edges(&self) -> usize {
        self.half_edges.len()
    }

    /// Get a borrowed reference to the patches
    pub fn patches(&self) -> &Vec<HePatch> {
        &self.patches
    }

    /// Get a borrowed reference to a patch by index
    pub fn patch(&self, index: usize) -> &HePatch {
        &self.patches[index]
    }

    /// Get the number of patches
    pub fn n_patches(&self) -> usize {
        self.patches.len()
    }

    /// Compute if the mesh is closed
    pub fn is_closed(&self) -> bool {
        for half_edge in self.half_edges.iter() {
            if half_edge.is_boundary() {
                return false;
            }
        }

        true
    }

    /// Compute if the mesh faces are consistently oriented
    pub fn is_consistent(&self) -> bool {
        for half_edge in self.half_edges.iter() {
            if let Some(twin) = half_edge.twin {
                if self.half_edges[twin].origin == half_edge.origin {
                    return false;
                }
            }
        }

        true
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct HeVertex {
    point: Vector3,
    half_edge: usize,
}

impl HeVertex {
    /// Get the point
    pub fn point(&self) -> Vector3 {
        self.point
    }

    /// Get the half edge handle
    pub fn half_edge(&self) -> usize {
        self.half_edge
    }
}

impl From<&Vertex> for HeVertex {
    fn from(vertex: &Vertex) -> HeVertex {
        HeVertex {
            point: (*vertex).into(),
            half_edge: 0,
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct HeFace {
    half_edge: usize,
    patch: Option<usize>,
}

impl HeFace {
    /// Construct a HeFace from its half edge and patch
    pub fn new(half_edge: usize, patch: Option<usize>) -> HeFace {
        HeFace { half_edge, patch }
    }

    /// Get the half edge handle
    pub fn half_edge(&self) -> usize {
        self.half_edge
    }

    /// Get the patch handle
    pub fn patch(&self) -> Option<usize> {
        self.patch
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct HeHalfEdge {
    origin: usize,
    face: usize,
    prev: usize,
    next: usize,
    twin: Option<usize>,
}

impl HeHalfEdge {
    /// Get the origin handle
    pub fn origin(&self) -> usize {
        self.origin
    }

    /// Get the face handle
    pub fn face(&self) -> usize {
        self.face
    }

    /// Get the previous half edge handle
    pub fn prev(&self) -> usize {
        self.prev
    }

    /// Get the next half edge handle
    pub fn next(&self) -> usize {
        self.next
    }

    /// Get the twin half edge handle
    pub fn twin(&self) -> Option<usize> {
        self.twin
    }

    /// Get if the the half edge is a boundary (no twin)
    pub fn is_boundary(&self) -> bool {
        self.twin.is_none()
    }
}

#[derive(Debug, Clone, Default)]
pub struct HePatch {
    name: String,
}

impl HePatch {
    /// Get a borrowed reference to the name
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl From<&Patch> for HePatch {
    fn from(patch: &Patch) -> HePatch {
        HePatch {
            name: patch.name().to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hemesh_from_obj() {
        let path = "tests/fixtures/box.obj";
        let mesh = HeMesh::from_obj(&path).unwrap();

        assert_eq!(mesh.n_vertices(), 8);
        assert_eq!(mesh.n_faces(), 12);
        assert_eq!(mesh.n_half_edges(), 36);
        assert_eq!(mesh.n_patches(), 0);
    }

    #[test]
    fn test_hemesh_from_obj_patches() {
        let path = "tests/fixtures/box_groups.obj";
        let mesh = HeMesh::from_obj(&path).unwrap();

        assert_eq!(mesh.n_vertices(), 8);
        assert_eq!(mesh.n_faces(), 12);
        assert_eq!(mesh.n_half_edges(), 36);
        assert_eq!(mesh.n_patches(), 6);
    }

    #[test]
    #[should_panic]
    fn test_hemesh_from_obj_nonmanifold() {
        let path = "tests/fixtures/box_nonmanifold.obj";
        HeMesh::from_obj(&path).unwrap();
    }
}
