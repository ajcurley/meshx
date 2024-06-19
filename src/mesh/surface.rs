use crate::geometry::{Aabb, Vector3};
use crate::mesh::common;
use crate::mesh::wavefront::ObjReader;

#[derive(Debug, Clone, Default)]
pub struct SurfaceMesh {
    vertices: Vec<Vertex>,
    faces: Vec<Face>,
    half_edges: Vec<HalfEdge>,
    edges: Vec<Edge>,
    patches: Vec<Patch>,
}

impl SurfaceMesh {
    /// Construct an SurfaceMesh from common mesh elements.
    pub fn new(
        _vertices: &[common::Vertex],
        _faces: &[common::Face],
        _patches: &[common::Patch],
    ) -> SurfaceMesh {
        unimplemented!();
    }

    /// Import a SurfaceMesh from an OBJ/WaveFront file
    pub fn from_obj(path: &str) -> std::io::Result<SurfaceMesh> {
        let mut reader = ObjReader::new(path);
        reader.read()?;

        let vertices = reader.vertices();
        let faces = reader.faces();
        let patches = reader.patches();
        let mesh = SurfaceMesh::new(vertices, faces, patches);

        Ok(mesh)
    }

    /// Export a SurfaceMesh to an OBJ/WaveFront file
    pub fn export_obj(_path: &str) -> std::io::Result<()> {
        unimplemented!();
    }

    /// Merge multiple SurfaceMeshes into a new surface mesh
    pub fn merge(_meshes: &[SurfaceMesh]) -> SurfaceMesh {
        unimplemented!()
    }

    /// Get a borrowed reference to the vertices
    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    /// Get the number of vertices
    pub fn number_of_vertices(&self) -> usize {
        self.vertices.len()
    }

    /// Get a borrowed reference to the faces
    pub fn faces(&self) -> &Vec<Face> {
        &self.faces
    }

    /// Get the number of faces
    pub fn number_of_faces(&self) -> usize {
        self.faces.len()
    }

    /// Get a borrowed reference to the half edges
    pub fn half_edges(&self) -> &Vec<HalfEdge> {
        &self.half_edges
    }

    /// Get the number of half edges
    pub fn number_of_half_edges(&self) -> usize {
        self.half_edges.len()
    }

    /// Get a borrowed reference to the edges
    pub fn edges(&self) -> &Vec<Edge> {
        &self.edges
    }

    /// Get the number of edges
    pub fn number_of_edges(&self) -> usize {
        self.edges.len()
    }

    /// Get a borrowed reference to the patches
    pub fn patches(&self) -> &Vec<Patch> {
        &self.patches
    }

    /// Get the number of patches
    pub fn number_of_patches(&self) -> usize {
        self.patches.len()
    }

    /// Get the neighboring vertices to a vertex
    pub fn vertex_neighbors(&self, _handle: VertexHandle) -> Vec<VertexHandle> {
        unimplemented!();
    }

    /// Get the faces using a vertex
    pub fn vertex_faces(&self, _handle: VertexHandle) -> Vec<FaceHandle> {
        unimplemented!();
    }

    /// Get the incoming half edges using a vertex
    pub fn vertex_incoming_half_edges(&self, _handle: VertexHandle) -> Vec<HalfEdgeHandle> {
        unimplemented!();
    }

    /// Get the outgoing half edges using a vertex
    pub fn vertex_outgoing_half_edges(&self, _handle: VertexHandle) -> Vec<HalfEdgeHandle> {
        unimplemented!();
    }

    /// Get the neighboring faces to a face
    pub fn face_neighbors(&self, handle: FaceHandle) -> Vec<FaceHandle> {
        let face = self[handle];
        let mut current = face.half_edge;
        let mut neighbors = vec![];

        loop {
            let half_edge = self[current];
            current = half_edge.next;

            if let Some(twin) = half_edge.twin {
                let neighbor = self[twin].face;
                neighbors.push(neighbor);
            }

            if current == face.half_edge {
                break;
            }
        }

        neighbors
    }

    /// Get the vertices of the face
    pub fn face_vertices(&self, handle: FaceHandle) -> Vec<VertexHandle> {
        let face = self[handle];
        let mut current = face.half_edge;
        let mut vertices = vec![];

        loop {
            let half_edge = self[current];
            vertices.push(half_edge.origin);
            current = half_edge.next;

            if current == face.half_edge {
                break;
            }
        }

        vertices
    }

    /// Get the half edges of the face
    pub fn face_half_edges(&self, handle: FaceHandle) -> Vec<HalfEdgeHandle> {
        let face = self[handle];
        let mut current = face.half_edge;
        let mut half_edges = vec![];

        loop {
            half_edges.push(current);
            current = self[current].next;

            if current == face.half_edge {
                break;
            }
        }

        half_edges
    }

    /// Compute the unit normal of the face using Newell's method.
    pub fn face_normal(&self, handle: FaceHandle) -> Vector3 {
        let mut normal = Vector3::zeros();
        let vertices = self.face_vertices(handle);
        let n = vertices.len();

        for i in 0..n {
            let j = vertices[i];
            let k = vertices[(i + 1) % n];
            let current = self[j].position;
            let next = self[k].position;

            normal[0] += (current[1] - next[1]) * (current[2] + next[2]);
            normal[1] += (current[2] - next[2]) * (current[0] + next[0]);
            normal[2] += (current[0] - next[0]) * (current[1] + next[1]);
        }

        normal.unit()
    }

    /// Compute the unit normals of all faces
    pub fn face_normals(&self) -> Vec<Vector3> {
        (0..self.number_of_faces())
            .map(|i| FaceHandle::new(i))
            .map(|h| self.face_normal(h))
            .collect()
    }

    /// Compute the axis-aligned bounding box
    pub fn aabb(&self) -> Aabb {
        let mut min = Vector3::ones() * std::f64::INFINITY;
        let mut max = Vector3::ones() * std::f64::NEG_INFINITY;

        for vertex in self.vertices.iter() {
            for i in 0..3 {
                if vertex.position[i] < min[i] {
                    min[i] = vertex.position[i];
                }

                if vertex.position[i] > max[i] {
                    max[i] = vertex.position[i];
                }
            }
        }

        Aabb::from_bounds(min, max)
    }

    /// Check if the surface mesh is closed
    pub fn is_closed(&self) -> bool {
        for half_edge in self.half_edges.iter() {
            if !half_edge.has_twin() {
                return false;
            }
        }

        true
    }

    /// Check if the faces of each component are oriented consistently
    pub fn is_consistent(&self) -> bool {
        for half_edge in self.half_edges.iter() {
            if let Some(twin) = half_edge.twin() {
                if half_edge.origin == self[twin].origin {
                    return false;
                }
            }
        }

        true
    }

    /// Check if the surface mesh consists of strictly triangle faces.
    pub fn is_triangle_mesh(&self) -> bool {
        unimplemented!();
    }

    /// Orient the faces of each component. This does not guarantee that
    /// all components are oriented consistently.
    pub fn orient(&mut self) {
        unimplemented!();
    }

    /// Orient the faces of all components consistently relative to a
    /// reference point external to the surface faces.
    pub fn orient_with_reference(&mut self, _point: Vector3) {
        unimplemented!();
    }

    /// Merge vertices within a tolerance. This will remove any degenerate
    /// faces resulting from merging vertices.
    pub fn merge_close_vertices(&mut self, _tolerance: f64) {
        unimplemented!();
    }

    /// Remove duplicate patches from the surface mesh.
    pub fn remove_duplicate_patches(&mut self) {
        unimplemented!();
    }

    /// Remove duplicate faces from the surface mesh.
    pub fn remove_duplicate_faces(&mut self) {
        unimplemented!();
    }

    /// Remove degenerate faces from the surface mesh. This may result in
    /// a surface mesh with open edges.
    pub fn remove_degenerate_faces(&mut self) {
        unimplemented!();
    }

    /// Prune the surace mesh to remove any unused/tombstoned vertices,
    /// faces, half edges, and patches.
    pub fn prune(&mut self) {
        unimplemented!();
    }

    /// Compute the components of contiguous faces.
    pub fn components(&self) -> Vec<FaceHandle> {
        unimplemented!();
    }

    /// Compute the edges whose adjacent faces form an angle greater than
    /// the threshold in radians.
    pub fn feature_edges(&self, _threshold: f64) -> Vec<EdgeHandle> {
        unimplemented!();
    }

    /// Compute the Gaussian curvature for each face.
    pub fn gaussian_curvature(&self) -> Vec<f64> {
        unimplemented!();
    }

    /// Scale the mesh by a constant factor.
    pub fn scale(&mut self, factor: f64) {
        for vertex in self.vertices.iter_mut() {
            vertex.position *= factor;
        }
    }
}

impl std::ops::Index<VertexHandle> for SurfaceMesh {
    type Output = Vertex;

    fn index(&self, handle: VertexHandle) -> &Self::Output {
        &self.vertices[handle.index()]
    }
}

impl std::ops::IndexMut<VertexHandle> for SurfaceMesh {
    fn index_mut(&mut self, handle: VertexHandle) -> &mut Self::Output {
        &mut self.vertices[handle.index()]
    }
}

impl std::ops::Index<FaceHandle> for SurfaceMesh {
    type Output = Face;

    fn index(&self, handle: FaceHandle) -> &Self::Output {
        &self.faces[handle.index()]
    }
}

impl std::ops::IndexMut<FaceHandle> for SurfaceMesh {
    fn index_mut(&mut self, handle: FaceHandle) -> &mut Self::Output {
        &mut self.faces[handle.index()]
    }
}

impl std::ops::Index<HalfEdgeHandle> for SurfaceMesh {
    type Output = HalfEdge;

    fn index(&self, handle: HalfEdgeHandle) -> &Self::Output {
        &self.half_edges[handle.index()]
    }
}

impl std::ops::IndexMut<HalfEdgeHandle> for SurfaceMesh {
    fn index_mut(&mut self, handle: HalfEdgeHandle) -> &mut Self::Output {
        &mut self.half_edges[handle.index()]
    }
}

impl std::ops::Index<EdgeHandle> for SurfaceMesh {
    type Output = Edge;

    fn index(&self, handle: EdgeHandle) -> &Self::Output {
        &self.edges[handle.index()]
    }
}

impl std::ops::IndexMut<EdgeHandle> for SurfaceMesh {
    fn index_mut(&mut self, handle: EdgeHandle) -> &mut Self::Output {
        &mut self.edges[handle.index()]
    }
}

impl std::ops::Index<PatchHandle> for SurfaceMesh {
    type Output = Patch;

    fn index(&self, handle: PatchHandle) -> &Self::Output {
        &self.patches[handle.index()]
    }
}

impl std::ops::IndexMut<PatchHandle> for SurfaceMesh {
    fn index_mut(&mut self, handle: PatchHandle) -> &mut Self::Output {
        &mut self.patches[handle.index()]
    }
}

impl Extract<FaceHandle> for SurfaceMesh {
    fn extract(&self, _handles: &[FaceHandle]) -> SurfaceMesh {
        unimplemented!();
    }
}

impl Extract<EdgeHandle> for SurfaceMesh {
    fn extract(&self, _handles: &[EdgeHandle]) -> SurfaceMesh {
        unimplemented!();
    }
}

impl Extract<PatchHandle> for SurfaceMesh {
    fn extract(&self, _handles: &[PatchHandle]) -> SurfaceMesh {
        unimplemented!();
    }
}

impl Flip<FaceHandle> for SurfaceMesh {
    fn flip(&mut self, _handle: FaceHandle) {
        unimplemented!();
    }
}

impl Flip<HalfEdgeHandle> for SurfaceMesh {
    fn flip(&mut self, _handle: HalfEdgeHandle) {
        unimplemented!();
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Vertex {
    position: Vector3,
    half_edge: HalfEdgeHandle,
}

impl Vertex {
    /// Get the position
    pub fn position(&self) -> Vector3 {
        self.position
    }

    /// Get the half edge handle
    pub fn half_edge(&self) -> HalfEdgeHandle {
        self.half_edge
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Hash)]
pub struct VertexHandle(usize);

impl VertexHandle {
    /// Construct a VertexHandle from an index
    pub fn new(index: usize) -> VertexHandle {
        VertexHandle(index)
    }

    /// Get the handle index
    pub fn index(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Face {
    half_edge: HalfEdgeHandle,
    patch: Option<PatchHandle>,
}

impl Face {
    /// Get the half edge handle
    pub fn half_edge(&self) -> HalfEdgeHandle {
        self.half_edge
    }

    /// Get the patch handle
    pub fn patch(&self) -> Option<PatchHandle> {
        self.patch
    }

    /// Check if the face has a patch
    pub fn has_patch(&self) -> bool {
        self.patch.is_some()
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Hash)]
pub struct FaceHandle(usize);

impl FaceHandle {
    /// Construct a FaceHandle from an index
    pub fn new(index: usize) -> FaceHandle {
        FaceHandle(index)
    }

    /// Get the handle index
    pub fn index(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct HalfEdge {
    origin: VertexHandle,
    face: FaceHandle,
    prev: HalfEdgeHandle,
    next: HalfEdgeHandle,
    twin: Option<HalfEdgeHandle>,
}

impl HalfEdge {
    /// Get the origin vertex handle
    pub fn origin(&self) -> VertexHandle {
        self.origin
    }

    /// Get the face handle
    pub fn face(&self) -> FaceHandle {
        self.face
    }

    /// Get the previous half edge handle
    pub fn prev(&self) -> HalfEdgeHandle {
        self.prev
    }

    /// Get the next half edge handle
    pub fn next(&self) -> HalfEdgeHandle {
        self.next
    }

    /// Get the twin half edge handle
    pub fn twin(&self) -> Option<HalfEdgeHandle> {
        self.twin
    }

    /// Check if the half edge has a twin
    pub fn has_twin(&self) -> bool {
        self.twin.is_some()
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Hash)]
pub struct HalfEdgeHandle(usize);

impl HalfEdgeHandle {
    /// Construct a HalfEdgeHandle from an index
    pub fn new(index: usize) -> HalfEdgeHandle {
        HalfEdgeHandle(index)
    }

    /// Get the handle index
    pub fn index(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Edge {
    half_edge: HalfEdgeHandle,
}

impl Edge {
    /// Get the half edge handle
    pub fn half_edge(&self) -> HalfEdgeHandle {
        self.half_edge
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Hash)]
pub struct EdgeHandle(usize);

impl EdgeHandle {
    /// Construct an EdgeHandle from an index
    pub fn new(index: usize) -> EdgeHandle {
        EdgeHandle(index)
    }

    /// Get the handle index
    pub fn index(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct Patch {
    name: String,
}

impl Patch {
    /// Get a borrowed reference to the name
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Hash)]
pub struct PatchHandle(usize);

impl PatchHandle {
    /// Construct a PatchHandle from an index
    pub fn new(index: usize) -> PatchHandle {
        PatchHandle(index)
    }

    /// Get the handle index
    pub fn index(&self) -> usize {
        self.0
    }
}

/// Extract a subset of the mesh
pub trait Extract<T> {
    fn extract(&self, handles: &[T]) -> SurfaceMesh;
}

/// Flip an element of the mesh
pub trait Flip<T> {
    fn flip(&mut self, handle: T);
}
