use std::collections::{HashMap, HashSet};

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

            let mut next_offset = (0..n).collect::<Vec<usize>>();
            let mut prev_offset = (0..n).collect::<Vec<usize>>();
            next_offset.rotate_left(1);
            prev_offset.rotate_right(1);

            for (edge_id, edge) in face.edges().iter().enumerate() {
                let half_edge_id = count + edge_id;
                let mut half_edge = HeHalfEdge::default();
                half_edge.origin = edge[0];
                half_edge.face = face_id;
                half_edge.prev = count + prev_offset[edge_id];
                half_edge.next = count + next_offset[edge_id];

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

    /// Compute the neighboring vertices for a vertex by index. This is only
    /// valid for closed oriented meshes.
    pub fn vertex_neighbors(&self, index: usize) -> Vec<usize> {
        let vertex = self.vertices[index];
        let mut current = vertex.half_edge;
        let mut neighbors = vec![];

        loop {
            let half_edge = self.half_edges[current];
            let prev = self.half_edges[half_edge.prev];
            neighbors.push(prev.origin);

            current = prev.twin.expect("mesh must be closed");

            if current == vertex.half_edge {
                break;
            }
        }

        neighbors
    }

    /// Compute the faces containing a vertex by index. This is only valid
    /// for closed oriented meshes.
    pub fn vertex_faces(&self, index: usize) -> Vec<usize> {
        let vertex = self.vertices[index];
        let mut current = vertex.half_edge;
        let mut faces = vec![];

        loop {
            let half_edge = self.half_edges[current];
            faces.push(half_edge.face);

            let prev = self.half_edges[half_edge.prev];
            current = prev.twin.expect("mesh must be closed");

            if current == vertex.half_edge {
                break;
            }
        }

        faces
    }

    /// Compute the vertices defining a face by index
    pub fn face_vertices(&self, index: usize) -> Vec<usize> {
        self.face_half_edges(index)
            .iter()
            .map(|&i| self.half_edges[i].origin)
            .collect()
    }

    /// Compute the neighboring faces for a face by index
    pub fn face_neighbors(&self, index: usize) -> Vec<usize> {
        self.face_half_edges(index)
            .iter()
            .map(|&i| self.half_edges[i])
            .filter(|h| !h.is_boundary())
            .map(|h| self.half_edges[h.twin.unwrap()].face)
            .collect()
    }

    /// Compute the ordered half edges defining the boundary of a face by index
    pub fn face_half_edges(&self, index: usize) -> Vec<usize> {
        let face = self.faces[index];
        let mut current = face.half_edge;
        let mut half_edges = vec![];

        loop {
            half_edges.push(current);
            current = self.half_edges[current].next;

            if current == face.half_edge {
                break;
            }
        }

        half_edges
    }

    /// Merge the mesh into the current mesh naively. This strictly copies
    /// the mesh and does not merge vertices, edges, or faces.
    pub fn merge(&mut self, other: &HeMesh) {
        let nv = self.n_vertices();
        let nf = self.n_faces();
        let nh = self.n_half_edges();
        let np = self.n_patches();

        for patch in other.patches() {
            let patch = patch.clone();
            self.patches.push(patch);
        }

        for vertex in other.vertices().iter() {
            let mut vertex = *vertex;
            vertex.half_edge += nh;
            self.vertices.push(vertex);
        }

        for face in other.faces().iter() {
            let mut face = *face;
            face.half_edge += nh;

            if let Some(patch) = face.patch {
                face.patch = Some(patch + np);
            }

            self.faces.push(face);
        }

        for half_edge in other.half_edges().iter() {
            let mut half_edge = *half_edge;
            half_edge.origin += nv;
            half_edge.face += nf;
            half_edge.prev += nh;
            half_edge.next += nh;

            if let Some(twin) = half_edge.twin {
                half_edge.twin = Some(twin + nh);
            }

            self.half_edges.push(half_edge)
        }
    }

    /// Combine patches with the same name explicitly.
    pub fn combine_patches(&mut self) {
        let mut patches = vec![];
        let mut index: HashMap<&str, usize> = HashMap::new();

        for (i, patch) in self.patches.iter().enumerate() {
            let name = patch.name();

            if !index.contains_key(name) {
                index.insert(name, i);
                patches.push(patch.clone());
            }
        }

        for face in self.faces.iter_mut() {
            if let Some(patch) = face.patch {
                let name = self.patches[patch].name();
                face.patch = Some(index[name]);
            }
        }

        self.patches = patches;
    }

    /// Extract a subset from the mesh by the index of the face. This
    /// copies the target subset into a new mesh.
    pub fn extract_faces(&self, _faces: &Vec<usize>) -> HeMesh {
        unimplemented!()
    }

    /// Extract a subset from the mesh by the patch names. This copies the
    /// target subset into a new mesh.
    pub fn extract_patches(&self, patches: &Vec<&str>) -> HeMesh {
        let mut selected = HashSet::new();
        let mut index = vec![false; self.n_patches()];
        let mut faces = vec![];

        for patch in patches.iter() {
            selected.insert(patch.to_string());
        }

        for (i, patch) in self.patches.iter().enumerate() {
            if selected.contains(patch.name()) {
                index[i] = true;
            }
        }

        for (i, face) in self.faces.iter().enumerate() {
            if let Some(patch) = face.patch {
                if index[patch] {
                    faces.push(i);
                }
            }
        }

        self.extract_faces(&faces)
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
    /// Construct a HeHalfEdge from its components
    pub fn new(
        origin: usize,
        face: usize,
        prev: usize,
        next: usize,
        twin: Option<usize>,
    ) -> HeHalfEdge {
        HeHalfEdge {
            origin,
            face,
            prev,
            next,
            twin,
        }
    }

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
    fn test_from_obj() {
        let path = "tests/fixtures/box.obj";
        let mesh = HeMesh::from_obj(&path).unwrap();

        assert_eq!(mesh.n_vertices(), 8);
        assert_eq!(mesh.n_faces(), 12);
        assert_eq!(mesh.n_half_edges(), 36);
        assert_eq!(mesh.n_patches(), 0);
    }

    #[test]
    fn test_from_obj_patches() {
        let path = "tests/fixtures/box_groups.obj";
        let mesh = HeMesh::from_obj(&path).unwrap();

        assert_eq!(mesh.n_vertices(), 8);
        assert_eq!(mesh.n_faces(), 12);
        assert_eq!(mesh.n_half_edges(), 36);
        assert_eq!(mesh.n_patches(), 6);
    }

    #[test]
    #[should_panic]
    fn test_from_obj_nonmanifold() {
        let path = "tests/fixtures/box_nonmanifold.obj";
        HeMesh::from_obj(&path).unwrap();
    }

    #[test]
    fn test_is_closed() {
        let path = "tests/fixtures/box.obj";
        let mesh = HeMesh::from_obj(&path).unwrap();

        assert!(mesh.is_closed());
    }

    #[test]
    fn test_is_closed_open() {
        let path = "tests/fixtures/box_open.obj";
        let mesh = HeMesh::from_obj(&path).unwrap();

        assert!(!mesh.is_closed());
    }

    #[test]
    fn test_is_consistent() {
        let path = "tests/fixtures/box.obj";
        let mesh = HeMesh::from_obj(&path).unwrap();

        assert!(mesh.is_consistent());
    }

    #[test]
    fn test_is_consistent_inverted() {
        let path = "tests/fixtures/box_inconsistent.obj";
        let mesh = HeMesh::from_obj(&path).unwrap();

        assert!(!mesh.is_consistent());
    }

    #[test]
    fn test_vertex_neighbors() {
        let path = "tests/fixtures/box.obj";
        let mesh = HeMesh::from_obj(&path).unwrap();

        let neighbors = mesh.vertex_neighbors(1);
        dbg!(&neighbors);

        assert_eq!(neighbors.len(), 5);
        assert_eq!(neighbors[0], 3);
        assert_eq!(neighbors[1], 2);
        assert_eq!(neighbors[2], 0);
        assert_eq!(neighbors[3], 4);
        assert_eq!(neighbors[4], 5);
    }

    #[test]
    #[ignore]
    fn test_vertex_neighbors_inverted() {
        // TODO: implement
    }

    #[test]
    fn test_vertex_faces() {
        let path = "tests/fixtures/box.obj";
        let mesh = HeMesh::from_obj(&path).unwrap();

        let faces = mesh.vertex_faces(1);

        assert_eq!(faces.len(), 5);
        assert_eq!(faces[0], 10);
        assert_eq!(faces[1], 1);
        assert_eq!(faces[2], 0);
        assert_eq!(faces[3], 4);
        assert_eq!(faces[4], 5);
    }

    #[test]
    #[ignore]
    fn test_vertex_faces_inverted() {
        // TODO: implement
    }

    #[test]
    #[should_panic]
    fn test_vertex_faces_open() {
        let path = "tests/fixtures/box_open.obj";
        let mesh = HeMesh::from_obj(&path).unwrap();

        mesh.vertex_faces(2);
    }

    #[test]
    fn test_face_neighbors() {
        let path = "tests/fixtures/box.obj";
        let mesh = HeMesh::from_obj(&path).unwrap();

        let neighbors = mesh.face_neighbors(1);

        assert_eq!(neighbors.len(), 3);
        assert_eq!(neighbors[0], 10);
        assert_eq!(neighbors[1], 6);
        assert_eq!(neighbors[2], 0);
    }

    #[test]
    fn test_face_half_edges() {
        let path = "tests/fixtures/box.obj";
        let mesh = HeMesh::from_obj(&path).unwrap();

        let half_edges = mesh.face_half_edges(1);

        assert_eq!(half_edges.len(), 3);
        assert_eq!(mesh.half_edge(half_edges[0]).origin, 1);
        assert_eq!(mesh.half_edge(half_edges[1]).origin, 3);
        assert_eq!(mesh.half_edge(half_edges[2]).origin, 2);
    }

    #[test]
    fn test_merge() {
        let path = "tests/fixtures/box.obj";
        let mut mesh1 = HeMesh::from_obj(&path).unwrap();
        let mesh2 = HeMesh::from_obj(&path).unwrap();

        mesh1.merge(&mesh2);

        assert_eq!(mesh1.n_vertices(), 16);
        assert_eq!(mesh1.n_faces(), 24);
        assert_eq!(mesh1.n_half_edges(), 72);
        assert_eq!(mesh1.n_patches(), 0);
    }

    #[test]
    fn test_combine_patches() {
        let path = "tests/fixtures/box_groups.obj";
        let mut mesh1 = HeMesh::from_obj(&path).unwrap();
        let mesh2 = HeMesh::from_obj(&path).unwrap();

        mesh1.merge(&mesh2);

        assert_eq!(mesh1.n_vertices(), 16);
        assert_eq!(mesh1.n_faces(), 24);
        assert_eq!(mesh1.n_half_edges(), 72);
        assert_eq!(mesh1.n_patches(), 12);

        mesh1.combine_patches();

        assert_eq!(mesh1.n_patches(), 6);
    }

    #[test]
    fn test_extract_faces() {
        let path = "tests/fixtures/box_groups.obj";
        let mesh1 = HeMesh::from_obj(&path).unwrap();

        let faces = vec![0, 1, 6];
        let mesh2 = mesh1.extract_faces(&faces);

        assert_eq!(mesh2.n_vertices(), 5);
        assert_eq!(mesh2.n_faces(), 3);
        assert_eq!(mesh2.n_half_edges(), 9);
        assert_eq!(mesh2.n_patches(), 2);
    }

    #[test]
    fn test_extract_patches() {
        let path = "tests/fixtures/box_groups.obj";
        let mesh1 = HeMesh::from_obj(&path).unwrap();

        let patches: Vec<&str> = vec!["front", "right"];
        let mesh2 = mesh1.extract_patches(&patches);

        assert_eq!(mesh2.n_vertices(), 5);
        assert_eq!(mesh2.n_faces(), 3);
        assert_eq!(mesh2.n_half_edges(), 9);
        assert_eq!(mesh2.n_patches(), 2);
    }
}
