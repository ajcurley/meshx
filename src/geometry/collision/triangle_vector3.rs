use crate::geometry::{Triangle, Vector3};

/// Check for a spatial intersection between the Triangle and Vector3
pub fn intersects_triangle_vector3(triangle: &Triangle, v: &Vector3) -> bool {
    let p = triangle.p();
    let q = triangle.q();
    let r = triangle.r();

    // Check if the point is inside the axis-aligned bounding box of the triangle
    // and if not, reject the intersection.
    for i in 0..3 {
        if v[i] > p[i].max(q[i]).max(r[i]) {
            return false;
        }

        if v[i] < p[i].min(q[i]).min(r[i]) {
            return false;
        }
    }

    // For each triangle side, make a vector out of it by subtracting the
    // vertices. Make another vector from one vertex to point v. The cross
    // product of these two vectors is orthogonal to both and the signs of
    // its components indicate whether v is inside or outside of the triangle.
    let vect12 = p - q;
    let vect1h = p - *v;
    let cross12_1p = Vector3::cross(&vect12, &vect1h);
    let sign12 = sign3(cross12_1p);

    let vect23 = q - r;
    let vect2h = q - *v;
    let cross23_2p = Vector3::cross(&vect23, &vect2h);
    let sign23 = sign3(cross23_2p);

    let vect31 = r - p;
    let vect3h = r - *v;
    let cross31_3p = Vector3::cross(&vect31, &vect3h);
    let sign31 = sign3(cross31_3p);

    sign12 & sign23 & sign31 != 0
}

fn sign3(a: Vector3) -> usize {
    const EPSILON: f64 = 1e-5;

    let mut sign: usize = 0;

    sign |= if a.x() < EPSILON { 4 } else { 0 };
    sign |= if a.x() > -EPSILON { 32 } else { 0 };
    sign |= if a.y() < EPSILON { 2 } else { 0 };
    sign |= if a.y() > -EPSILON { 16 } else { 0 };
    sign |= if a.z() < EPSILON { 1 } else { 0 };
    sign |= if a.z() > -EPSILON { 8 } else { 0 };

    sign
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_triangle() -> Triangle {
        let p = Vector3::new(0., 0., 0.);
        let q = Vector3::new(1., 0., 0.);
        let r = Vector3::new(1., 1., 0.);
        Triangle::new(p, q, r)
    }

    #[test]
    fn test_triangle_vector3_ok() {
        let triangle = get_triangle();
        let point = Vector3::new(0.9, 0.9, 0.);

        let intersects = intersects_triangle_vector3(&triangle, &point);

        assert!(intersects);
    }

    #[test]
    fn test_triangle_vector3_fail_above() {
        let triangle = get_triangle();
        let point = Vector3::new(0.9, 0.9, 0.1);

        let intersects = intersects_triangle_vector3(&triangle, &point);

        assert!(!intersects);
    }

    #[test]
    fn test_triangle_vector3_fail_below() {
        let triangle = get_triangle();
        let point = Vector3::new(0.9, 0.9, -0.1);

        let intersects = intersects_triangle_vector3(&triangle, &point);

        assert!(!intersects);
    }

    #[test]
    fn test_triangle_vector3_fail_beside() {
        let triangle = get_triangle();
        let point = Vector3::new(1.1, 1.1, 0.);

        let intersects = intersects_triangle_vector3(&triangle, &point);

        assert!(!intersects);
    }
}
