use crate::geometry::{Clip, Distance, Intersection, Line, Plane, Vector3};

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

    /// Get the lines defining the boundary.
    pub fn lines(&self) -> Vec<Line> {
        let mut lines = vec![];
        let n = self.vertices.len();

        for i in 0..n {
            let p = self.vertices[i];
            let q = self.vertices[(i + 1) % n];
            lines.push(Line::new(p, q));
        }

        lines
    }
}

impl Clip<Plane> for Polygon {
    type Output = Polygon;

    fn clip(&self, plane: &Plane) -> Option<Self::Output> {
        let mut vertices = vec![];

        for line in self.lines() {
            let d1 = plane.distance(&line.p());
            let d2 = plane.distance(&line.q());

            if d1 >= 0. && d2 >= 0. {
                vertices.push(line.p());
            } else if d1 <= 0. && d2 > 0. {
                let t = plane.intersection(&line);
                vertices.push(t?);
            } else if d1 > 0. && d2 <= 0. {
                let t = plane.intersection(&line);
                vertices.push(line.p());
                vertices.push(t?);
            }
        }

        if vertices.len() < 3 {
            return None;
        }

        Some(Polygon::new(vertices))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_clip_plane_polygon_ok_quad() {
        let p = Vector3::new(0., 0., 0.);
        let q = Vector3::new(1., 0., 0.);
        let r = Vector3::new(1., 1., 0.);
        let polygon = Polygon::new(vec![p, q, r]);

        let normal = Vector3::new(1., 0., 0.);
        let plane = Plane::new(normal, -0.5);

        let result = polygon.clip(&plane).unwrap();

        assert_eq!(result.vertices.len(), 4);
        assert_eq!(result.vertices[0], Vector3::new(0.5, 0., 0.));
        assert_eq!(result.vertices[1], Vector3::new(1., 0., 0.));
        assert_eq!(result.vertices[2], Vector3::new(1., 1., 0.));
        assert_eq!(result.vertices[3], Vector3::new(0.5, 0.5, 0.));
    }

    #[test]
    fn test_clip_plane_polygon_ok_triangle() {
        let p = Vector3::new(0., 0., 0.);
        let q = Vector3::new(1., 0., 0.);
        let r = Vector3::new(1., 1., 0.);
        let polygon = Polygon::new(vec![p, q, r]);

        let normal = Vector3::new(-1., 0., 0.);
        let plane = Plane::new(normal, 0.5);

        let result = polygon.clip(&plane).unwrap();

        assert_eq!(result.vertices.len(), 3);
        assert_eq!(result.vertices[0], Vector3::new(0., 0., 0.));
        assert_eq!(result.vertices[1], Vector3::new(0.5, 0., 0.));
        assert_eq!(result.vertices[2], Vector3::new(0.5, 0.5, 0.));
    }
}
