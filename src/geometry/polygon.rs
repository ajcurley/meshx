use crate::geometry::collision::{Clip, Distance, Intersection, Intersects};
use crate::geometry::{Aabb, Line, Plane, Triangle, Vector3};

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

    /// Compute the triangulation of the polygon.
    pub fn triangulate(&self) -> Vec<Triangle> {
        if self.vertices.len() < 3 {
            return vec![];
        }

        let mut triangles = vec![];
        let mut remaining: Vec<usize> = (0..self.vertices.len()).collect();

        while remaining.len() > 3 {
            let n = remaining.len();

            for i in 0..n {
                if self.is_ear(remaining[i]) {
                    let j = if i == 0 { n - 1 } else { (i - 1) % n };
                    let k = (i + 1) % n;

                    let p = self.vertices[remaining[j]];
                    let q = self.vertices[remaining[i]];
                    let r = self.vertices[remaining[k]];
                    let triangle = Triangle::new(p, q, r);

                    triangles.push(triangle);
                    remaining.remove(i);
                    break;
                }
            }
        }

        let p = self.vertices[remaining[0]];
        let q = self.vertices[remaining[1]];
        let r = self.vertices[remaining[2]];
        let triangle = Triangle::new(p, q, r);
        triangles.push(triangle);

        triangles
    }

    /// Check if the vertex is an ear for triangulation.
    fn is_ear(&self, index: usize) -> bool {
        // Compute the indices of the vertices defining the triangle
        let n = self.vertices.len();
        let pi = if index == 0 { n - 1 } else { (index - 1) % n };
        let qi = index;
        let ri = (index + 1) % n;

        let p = self.vertices[pi];
        let q = self.vertices[qi];
        let r = self.vertices[ri];

        // Check if the angle is convex at q
        let u = p - q;
        let v = r - q;

        if Vector3::angle(&u, &v) >= std::f64::consts::PI {
            return false;
        }

        // Check if any other point in the polygon lies inside the triangle
        let triangle = Triangle::new(p, q, r);

        for (j, point) in self.vertices.iter().enumerate() {
            if j != pi && j != qi && j != ri {
                if triangle.intersects(point) {
                    return false;
                }
            }
        }

        true
    }
}

impl std::ops::Index<usize> for Polygon {
    type Output = Vector3;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vertices[index]
    }
}

impl std::ops::IndexMut<usize> for Polygon {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vertices[index]
    }
}

impl Clip<Aabb> for Polygon {
    type Output = Polygon;

    fn clip(&self, aabb: &Aabb) -> Option<Self::Output> {
        let mut polygon = self.clone();

        for plane in aabb.planes() {
            if let Some(clipped) = polygon.clip(&plane) {
                polygon = clipped;
            } else {
                return None;
            }
        }

        Some(polygon)
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
    fn test_clip_polygon_plane_ok_quad() {
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
    fn test_clip_polygon_plane_ok_triangle() {
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

    #[test]
    fn test_clip_polygon_plane_ok_full() {
        let p = Vector3::new(0., 0., 0.);
        let q = Vector3::new(1., 0., 0.);
        let r = Vector3::new(1., 1., 0.);
        let polygon = Polygon::new(vec![p, q, r]);

        let normal = Vector3::new(-1., 0., 0.);
        let plane = Plane::new(normal, 2.);

        let result = polygon.clip(&plane).unwrap();

        assert_eq!(result.vertices.len(), 3);
        assert_eq!(result.vertices[0], p);
        assert_eq!(result.vertices[1], q);
        assert_eq!(result.vertices[2], r);
    }

    #[test]
    fn test_clip_polygon_plane_fail() {
        let p = Vector3::new(0., 0., 0.);
        let q = Vector3::new(1., 0., 0.);
        let r = Vector3::new(1., 1., 0.);
        let polygon = Polygon::new(vec![p, q, r]);

        let normal = Vector3::new(1., 0., 0.);
        let plane = Plane::new(normal, -2.);

        let result = polygon.clip(&plane);
        assert!(result.is_none());
    }

    #[test]
    fn test_clip_polygon_aabb_ok() {
        let p = Vector3::new(0., 0., 0.5);
        let q = Vector3::new(1., 0., 0.5);
        let r = Vector3::new(1., 1., 0.5);
        let polygon = Polygon::new(vec![p, q, r]);

        let result = polygon.clip(&Aabb::unit()).unwrap();

        assert_eq!(result.vertices.len(), 3);
        assert_eq!(result.vertices[0], Vector3::new(0., 0., 0.5));
        assert_eq!(result.vertices[1], Vector3::new(0.5, 0., 0.5));
        assert_eq!(result.vertices[2], Vector3::new(0.5, 0.5, 0.5));
    }

    #[test]
    fn test_clip_polygon_aabb_ok_quad() {
        let p = Vector3::new(0., 0., 0.5);
        let q = Vector3::new(1., 0., 0.5);
        let r = Vector3::new(1., 1., 0.5);
        let s = Vector3::new(0., 1., 0.5);
        let polygon = Polygon::new(vec![p, q, r, s]);

        let result = polygon.clip(&Aabb::unit()).unwrap();

        assert_eq!(result.vertices.len(), 4);
        assert_eq!(result.vertices[0], Vector3::new(0., 0., 0.5));
        assert_eq!(result.vertices[1], Vector3::new(0.5, 0., 0.5));
        assert_eq!(result.vertices[2], Vector3::new(0.5, 0.5, 0.5));
        assert_eq!(result.vertices[3], Vector3::new(0., 0.5, 0.5));
    }

    #[test]
    fn test_triangulate_polygon_convex() {
        let v0 = Vector3::new(0., 0., 0.);
        let v1 = Vector3::new(1., 0., 0.);
        let v2 = Vector3::new(2., 1., 0.);
        let v3 = Vector3::new(1.5, 1.5, 0.);
        let v4 = Vector3::new(-1., 1., 0.);

        let polygon = Polygon::new(vec![v0, v1, v2, v3, v4]);
        let t0 = Triangle::new(v4, v0, v1);
        let t1 = Triangle::new(v4, v1, v2);
        let t2 = Triangle::new(v2, v3, v4);

        let triangles = polygon.triangulate();

        assert_eq!(triangles.len(), 3);
        assert_eq!(triangles[0], t0);
        assert_eq!(triangles[1], t1);
        assert_eq!(triangles[2], t2);
    }

    #[test]
    fn test_triangulate_polygon_concave() {
        let v0 = Vector3::new(0., 0., 0.);
        let v1 = Vector3::new(1., 0., 0.);
        let v2 = Vector3::new(2., 1., 0.);
        let v3 = Vector3::new(1.5, 1.5, 0.);
        let v4 = Vector3::new(1.2, 0.6, 0.);

        let polygon = Polygon::new(vec![v0, v1, v2, v3, v4]);
        let t0 = Triangle::new(v4, v0, v1);
        let t1 = Triangle::new(v2, v3, v4);
        let t2 = Triangle::new(v1, v2, v4);

        let triangles = polygon.triangulate();

        assert_eq!(triangles.len(), 3);
        assert_eq!(triangles[0], t0);
        assert_eq!(triangles[1], t1);
        assert_eq!(triangles[2], t2);
    }

    #[test]
    fn test_triangulate_polygon_nonplanar() {
        let v0 = Vector3::new(0., 0., 0.);
        let v1 = Vector3::new(1., 0., 0.);
        let v2 = Vector3::new(1., 1., 1.);
        let v3 = Vector3::new(0., 1., 0.);

        let polygon = Polygon::new(vec![v0, v1, v2, v3]);

        let triangles = polygon.triangulate();
        let t0 = Triangle::new(v3, v0, v1);
        let t1 = Triangle::new(v1, v2, v3);

        assert_eq!(triangles.len(), 2);
        assert_eq!(triangles[0], t0);
        assert_eq!(triangles[1], t1);
    }
}
