use std::collections::{BTreeMap, BTreeSet};

use crate::mesh::Face;

/// Given a list of Faces, merge faces sharing at least one edge. For
/// any merged Faces, the patch will be that of the first Face in the
/// input list. This assumes that all faces are consistently oriented.
pub fn merge_faces(faces: &Vec<Face>) -> Face {
    let mut adjacency: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    let mut patch = None;

    for face in faces.iter() {
        if patch.is_none() {
            patch = face.patch();
        }

        for edge in face.edges() {
            let p = edge.p();
            let q = edge.q();
            let mut shared = false;

            if let Some(vertices) = adjacency.get_mut(&q) {
                if vertices.contains(&p) {
                    vertices.remove(&p);
                    shared = true;
                }

                if vertices.is_empty() {
                    adjacency.remove(&q);
                }
            }

            if !shared {
                if let Some(vertices) = adjacency.get_mut(&p) {
                    vertices.insert(q);
                } else {
                    adjacency.insert(p, BTreeSet::from([q]));
                }
            }
        }
    }

    let mut current: usize = *adjacency.keys().next().unwrap();
    let mut vertices = vec![current];

    while !adjacency.is_empty() {
        if let Some(nodes) = adjacency.remove(&current) {
            if nodes.len() != 1 {
                panic!("invalid polygon");
            }

            if let Some(&next) = nodes.iter().next() {
                if vertices.len() != 0 && next == vertices[0] {
                    break;
                }

                vertices.push(next);
                current = next;
            }
        }
    }

    if !adjacency.is_empty() {
        panic!("some vertices were not merged");
    }

    Face::new(vertices, patch)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_merge_faces_multiple() {
        let face0 = Face::new(vec![1, 2, 3], None);
        let face1 = Face::new(vec![2, 4, 3], None);
        let face2 = Face::new(vec![1, 3, 5], None);
        let face3 = Face::new(vec![1, 5, 6], None);

        let result = merge_faces(&vec![face0, face1, face2, face3]);
        let vertices = result.vertices();

        assert_eq!(vertices.len(), 6);
        assert_eq!(vertices[0], 1);
        assert_eq!(vertices[1], 2);
        assert_eq!(vertices[2], 4);
        assert_eq!(vertices[3], 3);
        assert_eq!(vertices[4], 5);
        assert_eq!(vertices[5], 6);
    }

    #[test]
    fn test_merge_faces_mixed() {
        let face0 = Face::new(vec![0, 1, 2], None);
        let face1 = Face::new(vec![2, 3, 4, 0], None);
        let face2 = Face::new(vec![2, 5, 4, 3], None);

        let result = merge_faces(&vec![face0, face1, face2]);
        let vertices = result.vertices();

        assert_eq!(vertices.len(), 5);
        assert_eq!(vertices[0], 0);
        assert_eq!(vertices[1], 1);
        assert_eq!(vertices[2], 2);
        assert_eq!(vertices[3], 5);
        assert_eq!(vertices[4], 4);
    }

    #[test]
    #[should_panic]
    fn test_merge_faces_invalid_orient() {
        let face0 = Face::new(vec![0, 1, 2], None);
        let face1 = Face::new(vec![1, 2, 3], None);

        merge_faces(&vec![face0, face1]);
    }

    #[test]
    #[should_panic]
    fn test_merge_faces_invalid_disconnected() {
        let face0 = Face::new(vec![0, 1, 2], None);
        let face1 = Face::new(vec![3, 4, 5], None);

        merge_faces(&vec![face0, face1]);
    }
}
