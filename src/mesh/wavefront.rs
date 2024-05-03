use std::fs::File;
use std::io::prelude::*;

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;

use crate::mesh::common::{Edge, Face, Patch, Vertex};
use crate::mesh::utils::is_gzip;

#[derive(Debug, Clone)]
pub struct ObjReader {
    filename: String,
    vertices: Vec<Vertex>,
    faces: Vec<Face>,
    patches: Vec<Patch>,
}

impl ObjReader {
    /// Construct an ObjReader
    pub fn new(filename: &str) -> ObjReader {
        ObjReader {
            filename: filename.to_string(),
            vertices: vec![],
            faces: vec![],
            patches: vec![],
        }
    }

    /// Get a borrowed reference to the vertices
    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    /// Get a borrowed reference to the faces
    pub fn faces(&self) -> &Vec<Face> {
        &self.faces
    }

    /// Get a borrowed reference to the patches
    pub fn patches(&self) -> &Vec<Patch> {
        &self.patches
    }

    /// Read the file contents
    pub fn read(&mut self) -> std::io::Result<()> {
        let mut contents = String::new();
        let mut file = File::open(&self.filename)?;

        if is_gzip(&self.filename) {
            let mut file = GzDecoder::new(file);
            file.read_to_string(&mut contents)?;
        } else {
            file.read_to_string(&mut contents)?;
        }

        for (count, edge) in contents.lines().enumerate() {
            let count = count + 1;
            let edge = edge.trim();
            let args = edge.splitn(2, char::is_whitespace).collect::<Vec<&str>>();

            let result = match args.first() {
                Some(&"v") => self.parse_vertex(&args[1], count),
                Some(&"f") => self.parse_face(&args[1], count),
                Some(&"g") => self.parse_patch(&args[1], count),
                _ => Ok(()),
            };

            if let Err(error) = result {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    error.to_string(),
                ));
            }
        }

        Ok(())
    }

    /// Parse a vertex from an entry
    fn parse_vertex(&mut self, entry: &str, count: usize) -> Result<(), ParseObjError> {
        let mut vertex = Vertex::default();
        let mut is_error = false;

        for (i, value) in entry.split_whitespace().enumerate() {
            if i > 3 {
                is_error = true;
                break;
            }

            if let Ok(v) = value.parse::<f64>() {
                if i < 3 {
                    vertex[i] = v;
                }
            }
        }

        if is_error {
            let context = format!("invalid vertex: {}", entry);
            let error = ParseObjError::new(context, count);
            return Err(error);
        }

        self.vertices.push(vertex);

        Ok(())
    }

    /// Parse a face from an entry
    fn parse_face(&mut self, entry: &str, count: usize) -> Result<(), ParseObjError> {
        let mut vertices = vec![];
        let mut patch = None;
        let mut is_error = false;

        for value in entry.split_whitespace() {
            let value = value.splitn(2, "/").next().unwrap();

            if let Ok(v) = value.parse::<usize>() {
                if v != 0 {
                    vertices.push(v - 1);
                } else {
                    is_error = true;
                    break;
                }
            } else {
                is_error = true;
                break;
            }
        }

        if is_error {
            let context = format!("invalid face: {}", entry);
            let error = ParseObjError::new(context, count);
            return Err(error);
        }

        if self.patches.len() != 0 {
            patch = Some(self.patches.len() - 1);
        }

        let face = Face::new(vertices, patch);
        self.faces.push(face);

        Ok(())
    }

    /// Parse a patch from an entry
    fn parse_patch(&mut self, entry: &str, _: usize) -> Result<(), ParseObjError> {
        let name = entry.trim().to_string();
        let patch = Patch::new(name);
        self.patches.push(patch);
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct ObjWriter {
    vertices: Vec<Vertex>,
    faces: Vec<Face>,
    edges: Vec<Edge>,
    patches: Vec<Patch>,
}

impl ObjWriter {
    /// Construct an ObjWriter
    pub fn new() -> ObjWriter {
        ObjWriter::default()
    }

    /// Set the vertices
    pub fn set_vertices(&mut self, vertices: Vec<Vertex>) {
        self.vertices = vertices;
    }

    /// Set the faces
    pub fn set_faces(&mut self, faces: Vec<Face>) {
        self.faces = faces;
    }

    /// Set the edges
    pub fn set_edges(&mut self, edges: Vec<Edge>) {
        self.edges = edges;
    }

    /// Set the patches
    pub fn set_patches(&mut self, patches: Vec<Patch>) {
        self.patches = patches;
    }

    /// Write the mesh to file
    pub fn write(&self, filename: &str) -> std::io::Result<()> {
        let mut data = String::new();
        let mut patch_faces: Vec<Vec<usize>> = vec![vec![]; self.patches.len() + 1];
        let mut patch_edges: Vec<Vec<usize>> = vec![vec![]; self.patches.len() + 1];

        // Assign the faces to a patch. If a face does not have a patch, assign
        // it to the default patch at index 0.
        for (i, face) in self.faces.iter().enumerate() {
            if let Some(patch) = face.patch() {
                patch_faces[patch + 1].push(i);
            } else {
                patch_faces[0].push(i);
            }
        }

        // Assign the edges to a patch. If a edge does not have a patch, assign
        // it to the default patch at index 0.
        for (i, edge) in self.edges.iter().enumerate() {
            if let Some(patch) = edge.patch() {
                patch_edges[patch + 1].push(i);
            } else {
                patch_edges[0].push(i);
            }
        }

        // Format all the vertices.
        for vertex in self.vertices.iter() {
            let entry = self.format_vertex(vertex);
            data.push_str(&entry);
        }

        // Format the faces for the default (unnamed) patch.
        for i in patch_faces[0].iter() {
            let entry = self.format_face(&self.faces[*i]);
            data.push_str(&entry);
        }

        // Format the edges for the default (unnamed) patch.
        for i in patch_edges[0].iter() {
            let entry = self.format_edge(&self.edges[*i]);
            data.push_str(&entry);
        }

        // For each patch, format the patch followed by all of its assigned
        // faces and edges.
        for (i, patch) in self.patches.iter().enumerate() {
            let entry = self.format_patch(patch);
            data.push_str(&entry);

            for j in patch_faces[i + 1].iter() {
                let entry = self.format_face(&self.faces[*j]);
                data.push_str(&entry);
            }

            for j in patch_edges[i + 1].iter() {
                let entry = self.format_edge(&self.edges[*j]);
                data.push_str(&entry);
            }
        }

        // Write the data to file.
        let mut file = File::create(filename)?;
        let content = data.as_bytes();

        if is_gzip(&filename) {
            let mut encoder = GzEncoder::new(&mut file, Compression::default());
            encoder.write_all(&content)?;
        } else {
            file.write_all(&content)?;
        }

        Ok(())
    }

    /// Format a vertex to an entry
    fn format_vertex(&self, vertex: &Vertex) -> String {
        format!("v {} {} {}\n", vertex[0], vertex[1], vertex[2])
    }

    /// Format a face to an entry
    fn format_face(&self, face: &Face) -> String {
        let vertices = face
            .vertices()
            .iter()
            .map(|v| (v + 1).to_string())
            .collect::<Vec<String>>()
            .join(" ");

        format!("f {}\n", vertices)
    }

    /// Format a edge to an entry
    fn format_edge(&self, edge: &Edge) -> String {
        format!("l {} {}\n", edge[0], edge[1])
    }

    /// Format a patch to an entry
    fn format_patch(&self, patch: &Patch) -> String {
        format!("g {}\n", patch.name())
    }
}

#[derive(Debug, Clone)]
pub struct ParseObjError {
    context: String,
    line_id: usize,
}

impl ParseObjError {
    /// Construct a ParseObjError
    pub fn new(context: String, line_id: usize) -> ParseObjError {
        ParseObjError { context, line_id }
    }
}

impl std::fmt::Display for ParseObjError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "edge {}: {}", self.line_id, self.context)
    }
}

impl std::error::Error for ParseObjError {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_obj_reader() {
        let path = "tests/fixtures/box.obj";
        let mut reader = ObjReader::new(&path);
        reader.read().unwrap();

        assert_eq!(reader.vertices().len(), 8);
        assert_eq!(reader.faces().len(), 12);
        assert_eq!(reader.patches().len(), 0);
    }

    #[test]
    fn test_obj_reader_gzip() {
        let path = "tests/fixtures/box.obj.gz";
        let mut reader = ObjReader::new(&path);
        reader.read().unwrap();

        assert_eq!(reader.vertices().len(), 8);
        assert_eq!(reader.faces().len(), 12);
        assert_eq!(reader.patches().len(), 0);
    }

    #[test]
    fn test_obj_reader_patches() {
        let path = "tests/fixtures/box_groups.obj";
        let mut reader = ObjReader::new(&path);
        reader.read().unwrap();

        assert_eq!(reader.vertices().len(), 8);
        assert_eq!(reader.faces().len(), 12);
        assert_eq!(reader.patches().len(), 6);
    }

    #[test]
    fn test_obj_writer() {
        let path = "tests/fixtures/box.obj";
        let mut reader = ObjReader::new(&path);
        reader.read().unwrap();

        let out_path = "/tmp/box.obj";
        let mut writer = ObjWriter::new();
        writer.set_vertices(reader.vertices);
        writer.set_faces(reader.faces);
        writer.set_patches(reader.patches);
        writer.write(out_path).unwrap();

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
    fn test_obj_writer_gzip() {
        let path = "tests/fixtures/box.obj";
        let mut reader = ObjReader::new(&path);
        reader.read().unwrap();

        let out_path = "/tmp/box.obj.gz";
        let mut writer = ObjWriter::new();
        writer.set_vertices(reader.vertices);
        writer.set_faces(reader.faces);
        writer.set_patches(reader.patches);
        writer.write(out_path).unwrap();

        let mut expected_content = String::new();
        let mut actual_content = String::new();

        File::open(&path)
            .unwrap()
            .read_to_string(&mut expected_content)
            .unwrap();

        GzDecoder::new(File::open(&out_path).unwrap())
            .read_to_string(&mut actual_content)
            .unwrap();

        assert_eq!(actual_content, expected_content);
    }

    #[test]
    fn test_obj_writer_patches() {
        let path = "tests/fixtures/box_groups.obj";
        let mut reader = ObjReader::new(&path);
        reader.read().unwrap();

        let out_path = "/tmp/box_groups.obj";
        let mut writer = ObjWriter::new();
        writer.set_vertices(reader.vertices);
        writer.set_faces(reader.faces);
        writer.set_patches(reader.patches);
        writer.write(out_path).unwrap();

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
}
