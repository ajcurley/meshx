use std::ffi::OsStr;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;

#[derive(Debug, Clone)]
pub struct ObjReader {
    filename: String,
    vertices: Vec<ObjVertex>,
    faces: Vec<ObjFace>,
    groups: Vec<ObjGroup>,
}

impl ObjReader {
    /// Construct an ObjReader
    pub fn new(filename: &str) -> ObjReader {
        ObjReader {
            filename: filename.to_string(),
            vertices: vec![],
            faces: vec![],
            groups: vec![],
        }
    }

    /// Get a borrowed reference to the vertices
    pub fn vertices(&self) -> &Vec<ObjVertex> {
        &self.vertices
    }

    /// Get a borrowed reference to the faces
    pub fn faces(&self) -> &Vec<ObjFace> {
        &self.faces
    }

    /// Get a borrowed reference to the groups
    pub fn groups(&self) -> &Vec<ObjGroup> {
        &self.groups
    }

    /// Get if the input file is compressed
    pub fn is_compressed(&self) -> bool {
        let path = Path::new(&self.filename);
        let extension = path.extension().and_then(OsStr::to_str);

        if let Some(ext) = extension {
            return ext.to_lowercase() == "gz";
        }

        false
    }

    /// Read the file contents
    pub fn read(&mut self) -> std::io::Result<()> {
        let mut contents = String::new();
        let mut file = File::open(&self.filename)?;

        if self.is_compressed() {
            let mut file = GzDecoder::new(file);
            file.read_to_string(&mut contents)?;
        } else {
            file.read_to_string(&mut contents)?;
        }

        for (count, line) in contents.lines().enumerate() {
            let count = count + 1;
            let line = line.trim();
            let args = line.splitn(2, char::is_whitespace).collect::<Vec<&str>>();

            let result = match args.first() {
                Some(&"v") => self.parse_vertex(&args[1], count),
                Some(&"f") => self.parse_face(&args[1], count),
                Some(&"g") => self.parse_group(&args[1], count),
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
        let mut vertex = ObjVertex::default();
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
        let mut face = ObjFace::default();
        let mut is_error = false;

        for value in entry.split_whitespace() {
            let value = value.splitn(2, "/").next().unwrap();

            if let Ok(v) = value.parse::<usize>() {
                if v != 0 {
                    face.vertices.push(v - 1);
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

        if self.groups.len() != 0 {
            face.group = Some(self.groups.len() - 1);
        }

        self.faces.push(face);

        Ok(())
    }

    /// Parse a group from an entry
    fn parse_group(&mut self, entry: &str, _: usize) -> Result<(), ParseObjError> {
        let name = entry.trim().to_string();
        let group = ObjGroup::new(name);
        self.groups.push(group);
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct ObjWriter {
    vertices: Vec<ObjVertex>,
    faces: Vec<ObjFace>,
    lines: Vec<ObjLine>,
    groups: Vec<ObjGroup>,
}

impl ObjWriter {
    /// Construct an ObjWriter
    pub fn new() -> ObjWriter {
        ObjWriter::default()
    }

    /// Set the vertices
    pub fn set_vertices(&mut self, vertices: Vec<ObjVertex>) {
        self.vertices = vertices;
    }

    /// Set the faces
    pub fn set_faces(&mut self, faces: Vec<ObjFace>) {
        self.faces = faces;
    }

    /// Set the lines
    pub fn set_lines(&mut self, lines: Vec<ObjLine>) {
        self.lines = lines;
    }

    /// Set the groups
    pub fn set_groups(&mut self, groups: Vec<ObjGroup>) {
        self.groups = groups;
    }

    /// Write the mesh to file
    pub fn write(&self, filename: &str) -> std::io::Result<()> {
        let mut data = String::new();
        let mut group_faces: Vec<Vec<usize>> = vec![vec![]; self.groups.len() + 1];
        let mut group_lines: Vec<Vec<usize>> = vec![vec![]; self.groups.len() + 1];

        // Assign the faces to a group. If a face does not have a group, assign
        // it to the default group at index 0.
        for (i, face) in self.faces.iter().enumerate() {
            if let Some(group) = face.group {
                group_faces[group + 1].push(i);
            } else {
                group_faces[0].push(i);
            }
        }

        // Assign the lines to a group. If a line does not have a group, assign
        // it to the default group at index 0.
        for (i, line) in self.lines.iter().enumerate() {
            if let Some(group) = line.group {
                group_lines[group + 1].push(i);
            } else {
                group_lines[0].push(i);
            }
        }

        // Format all the vertices.
        for vertex in self.vertices.iter() {
            let entry = self.format_vertex(vertex);
            data.push_str(&entry);
        }

        // Format the faces for the default (unnamed) group.
        for i in group_faces[0].iter() {
            let entry = self.format_face(&self.faces[*i]);
            data.push_str(&entry);
        }

        // Format the lines for the default (unnamed) group.
        for i in group_lines[0].iter() {
            let entry = self.format_line(&self.lines[*i]);
            data.push_str(&entry);
        }

        // For each group, format the group followed by all of its assigned
        // faces and lines.
        for (i, group) in self.groups.iter().enumerate() {
            let entry = self.format_group(group);
            data.push_str(&entry);

            for j in group_faces[i + 1].iter() {
                let entry = self.format_face(&self.faces[*j]);
                data.push_str(&entry);
            }

            for j in group_lines[i + 1].iter() {
                let entry = self.format_line(&self.lines[*j]);
                data.push_str(&entry);
            }
        }

        // Write the data to file.
        let mut file = File::create(filename)?;
        let content = data.as_bytes();

        if self.is_compressed(filename) {
            let mut encoder = GzEncoder::new(&mut file, Compression::default());
            encoder.write_all(&content)?;
        } else {
            file.write_all(&content)?;
        }

        Ok(())
    }

    /// Format a vertex to an entry
    fn format_vertex(&self, vertex: &ObjVertex) -> String {
        format!("v {} {} {}\n", vertex.x, vertex.y, vertex.z)
    }

    /// Format a face to an entry
    fn format_face(&self, face: &ObjFace) -> String {
        let vertices = face
            .vertices
            .iter()
            .map(|v| (v + 1).to_string())
            .collect::<Vec<String>>()
            .join(" ");

        format!("f {}\n", vertices)
    }

    /// Format a line to an entry
    fn format_line(&self, line: &ObjLine) -> String {
        let vertices = line
            .vertices
            .iter()
            .map(|v| (v + 1).to_string())
            .collect::<Vec<String>>()
            .join(" ");

        format!("l {}\n", vertices)
    }

    /// Format a group to an entry
    fn format_group(&self, group: &ObjGroup) -> String {
        format!("g {}\n", group.name)
    }

    /// Get if the input file is compressed
    fn is_compressed(&self, filename: &str) -> bool {
        let path = Path::new(filename);
        let extension = path.extension().and_then(OsStr::to_str);

        if let Some(ext) = extension {
            return ext.to_lowercase() == "gz";
        }

        false
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct ObjVertex {
    x: f64,
    y: f64,
    z: f64,
}

impl ObjVertex {
    /// Construct an ObjVertex from its components
    pub fn new(x: f64, y: f64, z: f64) -> ObjVertex {
        ObjVertex { x, y, z }
    }
}

impl std::ops::Index<usize> for ObjVertex {
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

impl std::ops::IndexMut<usize> for ObjVertex {
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
pub struct ObjFace {
    vertices: Vec<usize>,
    group: Option<usize>,
}

impl ObjFace {
    /// Construct an ObjFace from its vertices and group
    pub fn new(vertices: Vec<usize>, group: Option<usize>) -> ObjFace {
        ObjFace { vertices, group }
    }

    /// Get a borrowed reference to the vertices
    pub fn vertices(&self) -> &Vec<usize> {
        &self.vertices
    }

    /// Get the group
    pub fn group(&self) -> Option<usize> {
        self.group
    }
}

#[derive(Debug, Clone)]
pub struct ObjLine {
    vertices: Vec<usize>,
    group: Option<usize>,
}

impl ObjLine {
    /// Construct an ObjLine from its vertices and group
    pub fn new(vertices: Vec<usize>, group: Option<usize>) -> ObjLine {
        ObjLine { vertices, group }
    }

    /// Get a borrowed reference to the vertices
    pub fn vertices(&self) -> &Vec<usize> {
        &self.vertices
    }

    /// Get the group
    pub fn group(&self) -> Option<usize> {
        self.group
    }
}

#[derive(Debug, Clone)]
pub struct ObjGroup {
    name: String,
}

impl ObjGroup {
    /// Construct an ObjGroup from its name
    pub fn new(name: String) -> ObjGroup {
        ObjGroup { name }
    }

    /// Get a borrowed reference to the name
    pub fn name(&self) -> &str {
        &self.name
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
        write!(f, "line {}: {}", self.line_id, self.context)
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
        assert_eq!(reader.groups().len(), 0);
    }

    #[test]
    fn test_obj_reader_gzip() {
        let path = "tests/fixtures/box.obj.gz";
        let mut reader = ObjReader::new(&path);
        reader.read().unwrap();

        assert_eq!(reader.vertices().len(), 8);
        assert_eq!(reader.faces().len(), 12);
        assert_eq!(reader.groups().len(), 0);
    }

    #[test]
    fn test_obj_reader_groups() {
        let path = "tests/fixtures/box_groups.obj";
        let mut reader = ObjReader::new(&path);
        reader.read().unwrap();

        assert_eq!(reader.vertices().len(), 8);
        assert_eq!(reader.faces().len(), 12);
        assert_eq!(reader.groups().len(), 6);
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
        writer.set_groups(reader.groups);
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
        writer.set_groups(reader.groups);
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
    fn test_obj_writer_groups() {
        let path = "tests/fixtures/box_groups.obj";
        let mut reader = ObjReader::new(&path);
        reader.read().unwrap();

        let out_path = "/tmp/box_groups.obj";
        let mut writer = ObjWriter::new();
        writer.set_vertices(reader.vertices);
        writer.set_faces(reader.faces);
        writer.set_groups(reader.groups);
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
