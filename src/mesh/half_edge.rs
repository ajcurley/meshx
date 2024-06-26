use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

use crate::geometry::{Aabb, Polygon, Sphere, Vector3, EPSILON};
use crate::mesh::wavefront::{ObjReader, ObjWriter};
use crate::mesh::{Face, Patch, Vertex};
use crate::spatial::{Octree, SearchMany};

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
        let mut half_edges: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

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
                    .entry(edge.as_tuple())
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

    /// Construct a HeMesh from a slice of Polygons. This will not remove the
    /// duplicate vertices.
    pub fn from_polygons(polygons: &[Polygon]) -> HeMesh {
        let mut vertices = vec![];
        let mut faces = vec![];
        let patches = vec![];

        for polygon in polygons.iter() {
            let mut face_vertices = vec![];

            for vertex in polygon.vertices().iter() {
                let n = vertices.len();
                face_vertices.push(n);

                let vertex = Vertex::from(*vertex);
                vertices.push(vertex);
            }

            let face = Face::new(face_vertices, None);
            faces.push(face);
        }

        HeMesh::new(&vertices, &faces, &patches)
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

    /// Export a HeMesh to an OBJ file
    pub fn export_obj(&self, filename: &str) -> std::io::Result<()> {
        let mut vertices = vec![];
        let mut faces = vec![];
        let mut patches = vec![];

        for vertex in self.vertices.iter() {
            let vertex = Vertex::from(vertex.point);
            vertices.push(vertex);
        }

        for (i, face) in self.faces.iter().enumerate() {
            let vertices = self.face_vertices(i);
            let face = Face::new(vertices, face.patch);
            faces.push(face);
        }

        for patch in self.patches.iter() {
            let name = patch.name().to_string();
            let patch = Patch::new(name);
            patches.push(patch);
        }

        let mut writer = ObjWriter::new();
        writer.set_vertices(vertices);
        writer.set_faces(faces);
        writer.set_patches(patches);
        writer.write(filename)
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

    /// Compute the axis-aligned bounding box
    pub fn aabb(&self) -> Aabb {
        let mut min = Vector3::ones() * std::f64::INFINITY;
        let mut max = Vector3::ones() * std::f64::NEG_INFINITY;

        for vertex in self.vertices.iter() {
            for i in 0..3 {
                if vertex.point[i] < min[i] {
                    min[i] = vertex.point[i]
                }

                if vertex.point[i] > max[i] {
                    max[i] = vertex.point[i];
                }
            }
        }

        Aabb::from_bounds(min, max)
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

    /// Compute if the neighboring pair of mesh faces are consistently
    /// oriented. If the faces do not share an edge, return false.
    pub fn is_consistent_faces(&self, i: usize, j: usize) -> bool {
        let mut index = HashSet::new();

        for k in self.face_half_edges(i) {
            index.insert(k);
        }

        for k in self.face_half_edges(j) {
            let half_edge = &self.half_edges[k];

            if let Some(twin) = half_edge.twin {
                if index.contains(&twin) {
                    let twin = &self.half_edges[twin];
                    return half_edge.origin != twin.origin;
                }
            }
        }

        false
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

    /// Compute the unit normal vector of a face.
    pub fn face_normal(&self, index: usize) -> Vector3 {
        let mut normal = Vector3::zeros();
        let index = self.face_vertices(index);
        let n = index.len();

        for i in 0..n {
            let p = self.vertices[index[i]].point;
            let q = self.vertices[index[(i + 1) % n]].point;
            normal += Vector3::cross(&p, &q);
        }

        normal.unit()
    }

    /// Compute the unit normals for all faces.
    pub fn face_normals(&self) -> Vec<Vector3> {
        (0..self.n_faces()).map(|i| self.face_normal(i)).collect()
    }

    /// Compute the feature edges using a threshold angle in radians. This will
    /// return the pair of half edges defining the edge.
    pub fn feature_edges(&self, angle: f64) -> Vec<(usize, usize)> {
        let mut visited = vec![false; self.n_half_edges()];
        let mut features = vec![];

        for (i, half_edge) in self.half_edges.iter().enumerate() {
            if !visited[i] {
                visited[i] = true;

                if let Some(j) = half_edge.twin {
                    visited[j] = true;

                    let twin = self.half_edges[j];
                    let u = self.face_normal(half_edge.face);
                    let v = self.face_normal(twin.face);

                    if Vector3::angle(&u, &v) >= angle {
                        features.push((i, j));
                    }
                }
            }
        }

        features
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

    /// Merge vertices within the geometric tolerance. This may result in a
    /// non-manifold mesh.
    pub fn merge_vertices(&mut self) {
        let aabb = self.aabb();
        let mut octree = Octree::<Vector3>::new(aabb);
        let mut queries = vec![];

        for vertex in self.vertices.iter() {
            octree.insert(vertex.point);

            let query = Sphere::new(vertex.point, EPSILON);
            queries.push(query);
        }

        let mut indices = BTreeMap::new();
        let mut lookup = HashMap::new();
        let mut edges: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

        for (i, items) in octree.search_many(&queries).iter().enumerate() {
            let index = items.iter().min().unwrap_or(&i);
            indices.insert(*index, 0);
            lookup.insert(i, *index);
        }

        for (i, (index, value)) in indices.iter_mut().enumerate() {
            self.vertices[i] = self.vertices[*index];
            *value = i;
        }

        for half_edge in self.half_edges.iter_mut() {
            (*half_edge).origin = indices[&lookup[&half_edge.origin]];
        }

        for (i, half_edge) in self.half_edges.iter().enumerate() {
            if half_edge.is_boundary() {
                let j = half_edge.origin;
                let k = self.half_edges[half_edge.next].origin;

                edges
                    .entry((j.min(k), j.max(k)))
                    .and_modify(|p| p.push(i))
                    .or_insert(vec![i]);
            }
        }

        for (_, shared) in edges.iter() {
            if shared.len() > 2 {
                panic!("non-manifold mesh");
            }

            if shared.len() == 2 {
                self.half_edges[shared[0]].twin = Some(shared[1]);
                self.half_edges[shared[1]].twin = Some(shared[0]);
            }
        }

        self.vertices.truncate(indices.len());
    }

    /// Combine patches with the same name explicitly.
    pub fn remove_duplicate_patches(&mut self) {
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
    pub fn extract_faces(&self, face_ids: &Vec<usize>) -> HeMesh {
        let mut faces = Vec::<Face>::with_capacity(face_ids.len());
        let mut vertices = vec![];
        let mut patches = vec![];
        let mut index_vertices = HashMap::new();
        let mut index_patches = HashMap::new();

        for &face_id in face_ids.iter() {
            let mut vertices_ = self.face_vertices(face_id);
            let mut patch_ = None;

            for old_id in vertices_.iter_mut() {
                if !index_vertices.contains_key(old_id) {
                    let new_id = index_vertices.len();
                    index_vertices.insert(*old_id, new_id);

                    let point = self.vertices[*old_id].point;
                    let vertex = Vertex::from(point);
                    vertices.push(vertex);
                }

                *old_id = index_vertices[old_id];
            }

            if let Some(old_id) = self.faces[face_id].patch {
                if !index_patches.contains_key(&old_id) {
                    let new_id = index_patches.len();
                    index_patches.insert(old_id, new_id);

                    let name = self.patches[old_id].name().to_string();
                    let patch = Patch::new(name);
                    patches.push(patch);
                }

                patch_ = Some(index_patches[&old_id]);
            }

            let face = Face::new(vertices_, patch_);
            faces.push(face);
        }

        HeMesh::new(&vertices, &faces, &patches)
    }

    /// Extract a subset from the mesh by the patch names. This copies the
    /// target subset into a new mesh.
    pub fn extract_patches(&self, patches: &Vec<String>) -> HeMesh {
        let mut selected = HashSet::new();
        let mut index = vec![false; self.n_patches()];
        let mut faces = vec![];

        for patch in patches.iter() {
            selected.insert(patch.clone());
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

    /// Orient the mesh such that the faces in each component have the same
    /// directed normal relative to each other. This does not ensure that the
    /// components' orientation are consistent.
    pub fn orient(&mut self) -> usize {
        let mut oriented = vec![false; self.n_faces()];
        let mut count = 0;

        for component in self.components() {
            let next = component[0];
            let mut queue = VecDeque::from([next]);

            while let Some(current) = queue.pop_front() {
                if !oriented[current] {
                    oriented[current] = true;

                    for neighbor in self.face_neighbors(current) {
                        if !oriented[neighbor] {
                            queue.push_back(neighbor);

                            if !self.is_consistent_faces(current, neighbor) {
                                self.flip_face(neighbor);
                                count += 1;
                            }
                        }
                    }
                }
            }
        }

        count
    }

    /// Compute the faces for each contiguous component in the mesh.
    pub fn components(&self) -> Vec<Vec<usize>> {
        let mut components = vec![];
        let mut visited = vec![false; self.n_faces()];

        for next in 0..visited.len() {
            if !visited[next] {
                let mut queue = VecDeque::from([next]);
                let mut component = vec![];

                while let Some(current) = queue.pop_front() {
                    if !visited[current] {
                        visited[current] = true;
                        component.push(current);

                        for neighbor in self.face_neighbors(current) {
                            if !visited[neighbor] {
                                queue.push_back(neighbor);
                            }
                        }
                    }
                }

                components.push(component);
            }
        }

        components
    }

    /// Split the mesh by feature angle (in radians).
    pub fn split_by_features(&self, angle: f64) -> Vec<Vec<usize>> {
        let mut components = vec![];
        let mut visited = vec![false; self.n_faces()];
        let normals = self.face_normals();

        for next in 0..visited.len() {
            if !visited[next] {
                let mut queue = VecDeque::from([next]);
                let mut component = vec![];

                while let Some(current) = queue.pop_front() {
                    if !visited[current] {
                        visited[current] = true;
                        component.push(current);

                        for neighbor in self.face_neighbors(current) {
                            let u = &normals[current];
                            let v = &normals[neighbor];

                            if !visited[neighbor] && Vector3::dot(&u, &v).acos() < angle {
                                queue.push_back(neighbor);
                            }
                        }
                    }
                }

                components.push(component);
            }
        }

        components
    }

    /// Flip the orientation of a face. This reverses the direction of all
    /// half edges for the face.
    pub fn flip_face(&mut self, index: usize) {
        self.face_half_edges(index)
            .iter()
            .for_each(|&i| self.flip_half_edge(i));
    }

    /// Flip the orientation of a half edge.
    pub fn flip_half_edge(&mut self, index: usize) {
        let half_edge = self.half_edges[index];
        let prev = half_edge.next;
        let origin = self.half_edges[prev].origin;

        self.half_edges[index].next = half_edge.prev;
        self.half_edges[index].prev = prev;
        self.half_edges[index].origin = origin;
    }

    /// Calculate the Gaussian curvature at a vertex. This assumes the mesh
    /// is composed of strictly trianglar faces and is oriented.
    pub fn curvature(&self, index: usize) -> f64 {
        let vertex = &self.vertices[index];
        let mut current = vertex.half_edge;
        let mut angle = 2. * std::f64::consts::PI;
        let mut area = 0.;

        loop {
            let half_edge = &self.half_edges[current];
            let next = &self.half_edges[half_edge.next];
            let prev = &self.half_edges[half_edge.prev];

            let p = self.vertices[prev.origin].point;
            let q = vertex.point;
            let r = self.vertices[next.origin].point;

            let u = p - q;
            let v = r - q;
            let theta = Vector3::angle(&u, &v);

            angle -= theta;
            area += Vector3::cross(&u, &v).mag() * 0.5;

            let twin = half_edge.twin.expect("mesh must be closed");
            current = self.half_edges[twin].next;

            if current == vertex.half_edge {
                break;
            }
        }

        3. * angle / area
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
    use std::fs::File;
    use std::io::prelude::*;

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
    fn test_export_obj() {
        let path = "tests/fixtures/box.obj";
        let mesh = HeMesh::from_obj(&path).unwrap();

        let out_path = "/tmp/test_export_obj.obj";
        mesh.export_obj(&out_path).unwrap();

        let mut expected_content = String::new();
        let mut actual_content = String::new();

        File::open(&path)
            .unwrap()
            .read_to_string(&mut expected_content)
            .unwrap();

        File::open(&out_path)
            .unwrap()
            .read_to_string(&mut actual_content)
            .unwrap();

        assert_eq!(actual_content, expected_content);
    }

    #[test]
    fn test_aabb() {
        let path = "tests/fixtures/box.obj";
        let mesh = HeMesh::from_obj(&path).unwrap();

        let aabb = mesh.aabb();

        assert_eq!(aabb.min(), Vector3::new(-0.5, -0.5, -0.5));
        assert_eq!(aabb.max(), Vector3::new(0.5, 0.5, 0.5));
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
    fn test_face_normal() {
        let path = "tests/fixtures/box.obj";
        let mesh = HeMesh::from_obj(&path).unwrap();

        let normal = mesh.face_normal(0);

        assert_eq!(normal, Vector3::new(-1., 0., 0.));
    }

    #[test]
    fn test_face_normal_polygon() {
        let path = "tests/fixtures/box_quads.obj";
        let mesh = HeMesh::from_obj(&path).unwrap();

        let normal = mesh.face_normal(0);

        assert_eq!(normal, Vector3::new(-1., 0., 0.));
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
    fn test_remove_duplicate_patches() {
        let path = "tests/fixtures/box_groups.obj";
        let mut mesh1 = HeMesh::from_obj(&path).unwrap();
        let mesh2 = HeMesh::from_obj(&path).unwrap();

        mesh1.merge(&mesh2);

        assert_eq!(mesh1.n_vertices(), 16);
        assert_eq!(mesh1.n_faces(), 24);
        assert_eq!(mesh1.n_half_edges(), 72);
        assert_eq!(mesh1.n_patches(), 12);

        mesh1.remove_duplicate_patches();

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

        let patches: Vec<String> = vec!["front".to_string(), "right".to_string()];
        let mesh2 = mesh1.extract_patches(&patches);

        assert_eq!(mesh2.n_vertices(), 6);
        assert_eq!(mesh2.n_faces(), 4);
        assert_eq!(mesh2.n_half_edges(), 12);
        assert_eq!(mesh2.n_patches(), 2);
    }

    #[test]
    fn test_components() {
        let path = "tests/fixtures/box.obj";
        let mesh = HeMesh::from_obj(&path).unwrap();

        let components = mesh.components();

        assert_eq!(components.len(), 1);
        assert_eq!(components[0].len(), mesh.n_faces());
    }

    #[test]
    fn test_components_multi() {
        let path = "tests/fixtures/box.obj";
        let mesh1 = HeMesh::from_obj(path).unwrap();

        let path = "tests/fixtures/sphere.obj";
        let mesh2 = HeMesh::from_obj(path).unwrap();

        let mut mesh3 = mesh1.clone();
        mesh3.merge(&mesh2);
        let components = mesh3.components();

        assert_eq!(components.len(), 2);
        assert_eq!(components[0].len(), mesh1.n_faces());
        assert_eq!(components[1].len(), mesh2.n_faces());
    }

    #[test]
    fn test_orient() {
        let path = "tests/fixtures/box_inconsistent.obj";
        let mut mesh = HeMesh::from_obj(&path).unwrap();

        assert!(!mesh.is_consistent());

        let count = mesh.orient();

        assert!(mesh.is_consistent());
        assert_eq!(count, 3);
    }

    #[test]
    fn test_orient_consistent() {
        let path = "tests/fixtures/box.obj";
        let mut mesh = HeMesh::from_obj(&path).unwrap();

        assert!(mesh.is_consistent());

        let count = mesh.orient();

        assert!(mesh.is_consistent());
        assert_eq!(count, 0);
    }

    #[test]
    fn test_feature_edges() {
        let path = "tests/fixtures/box.obj";
        let mesh = HeMesh::from_obj(&path).unwrap();

        let angle = 30. * std::f64::consts::PI / 180.;
        let features = mesh.feature_edges(angle);

        assert_eq!(features.len(), 12);
    }

    #[test]
    fn test_feature_edges_polygon() {
        let path = "tests/fixtures/box_quads.obj";
        let mesh = HeMesh::from_obj(&path).unwrap();

        let angle = 30. * std::f64::consts::PI / 180.;
        let features = mesh.feature_edges(angle);

        assert_eq!(features.len(), 12);
    }

    #[test]
    fn test_split_by_features_box_triangles() {
        let path = "tests/fixtures/box.obj";
        let mesh = HeMesh::from_obj(&path).unwrap();

        let angle = 30. * std::f64::consts::PI / 180.;
        let components = mesh.split_by_features(angle);

        assert_eq!(components.len(), 6);
        assert_eq!(components[0], vec![0, 1]);
        assert_eq!(components[1], vec![2, 3]);
        assert_eq!(components[2], vec![4, 5]);
        assert_eq!(components[3], vec![6, 7]);
        assert_eq!(components[4], vec![8, 9]);
        assert_eq!(components[5], vec![10, 11]);
    }

    #[test]
    fn test_split_by_features_box_quads() {
        let path = "tests/fixtures/box_quads.obj";
        let mesh = HeMesh::from_obj(&path).unwrap();

        let angle = 30. * std::f64::consts::PI / 180.;
        let components = mesh.split_by_features(angle);

        assert_eq!(components.len(), 6);
        assert_eq!(components[0], vec![0]);
        assert_eq!(components[1], vec![1]);
        assert_eq!(components[2], vec![2]);
        assert_eq!(components[3], vec![3]);
        assert_eq!(components[4], vec![4]);
        assert_eq!(components[5], vec![5]);
    }

    #[test]
    fn test_split_by_features_sphere() {
        let path = "tests/fixtures/sphere.obj";
        let mesh = HeMesh::from_obj(&path).unwrap();

        let angle = 30. * std::f64::consts::PI / 180.;
        let components = mesh.split_by_features(angle);

        assert_eq!(components.len(), 1);
    }

    #[test]
    fn test_curvature_sphere() {
        let path = "tests/fixtures/sphere.obj";
        let mesh = HeMesh::from_obj(&path).unwrap();

        let indices = vec![0, 14, 34];
        let expected = vec![3.62774, 4.64894, 4.18384];

        for (i, index) in indices.iter().enumerate() {
            let curvature = mesh.curvature(*index);
            let error = (curvature - expected[i]).abs();
            assert!(error <= 1e-5);
        }
    }

    #[test]
    fn test_merge_vertices() {
        let path = "tests/fixtures/polygons.obj";
        let mut mesh = HeMesh::from_obj(&path).unwrap();

        assert_eq!(mesh.n_vertices(), 214);
        assert_eq!(mesh.n_faces(), 59);
        assert_eq!(mesh.components().len(), 59);

        mesh.merge_vertices();

        assert_eq!(mesh.n_vertices(), 85);
        assert_eq!(mesh.n_faces(), 59);
        assert_eq!(mesh.components().len(), 1);
    }
}
