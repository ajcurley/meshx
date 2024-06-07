use crate::geometry::{Aabb, Triangle, Vector3};

/// Check for a spatial intersection between an Aabb and Triangle
pub fn intersects_aabb_triangle(aabb: &Aabb, triangle: &Triangle) -> bool {
    // Shift the system such that tha AABB center is at the origin
    let center = aabb.center();
    let halfsize = aabb.halfsize();
    let v0 = triangle.p() - center;
    let v1 = triangle.q() - center;
    let v2 = triangle.r() - center;

    // Compute the triangle edges e0, e1, e2
    let e0 = v1 - v0;
    let e1 = v2 - v1;
    let e2 = v0 - v2;

    // Bullet #3 - 9 tests
    let fex = e0.x().abs();
    let fey = e0.y().abs();
    let fez = e0.z().abs();

    if !axistest_x01(e0.z(), e0.y(), fez, fey, v0, v2, halfsize) {
        return false;
    }

    if !axistest_y02(e0.z(), e0.x(), fez, fex, v0, v2, halfsize) {
        return false;
    }

    if !axistest_z12(e0.y(), e0.x(), fey, fex, v1, v2, halfsize) {
        return false;
    }

    let fex = e1.x().abs();
    let fey = e1.y().abs();
    let fez = e1.z().abs();

    if !axistest_x01(e1.z(), e1.y(), fez, fey, v0, v2, halfsize) {
        return false;
    }

    if !axistest_y02(e1.z(), e1.x(), fez, fex, v0, v2, halfsize) {
        return false;
    }

    if !axistest_z0(e1.y(), e1.x(), fey, fex, v0, v1, halfsize) {
        return false;
    }

    let fex = e2.x().abs();
    let fey = e2.y().abs();
    let fez = e2.z().abs();

    if !axistest_x2(e2.z(), e2.y(), fez, fey, v0, v1, halfsize) {
        return false;
    }

    if !axistest_y1(e2.z(), e2.x(), fez, fex, v0, v1, halfsize) {
        return false;
    }

    if !axistest_z12(e2.y(), e2.x(), fey, fex, v1, v2, halfsize) {
        return false;
    }

    // Bullet #1 - Test the AABB against the minimum AABB of the triangle
    let (min, max) = findminmax(v0.x(), v1.x(), v2.x());

    if min > halfsize.x() || max < -halfsize.x() {
        return false;
    }

    let (min, max) = findminmax(v0.y(), v1.y(), v2.y());

    if min > halfsize.y() || max < -halfsize.y() {
        return false;
    }

    let (min, max) = findminmax(v0.z(), v1.z(), v2.z());

    if min > halfsize.z() || max < -halfsize.z() {
        return false;
    }

    // Bullet #2 - Test the triangle plane against the AABB
    let normal = Vector3::cross(&e0, &e1);

    plane_box_overlap(normal, v0, halfsize)
}

fn axistest_x01(a: f64, b: f64, fa: f64, fb: f64, v0: Vector3, v2: Vector3, h: Vector3) -> bool {
    let p0 = a * v0.y() - b * v0.z();
    let p2 = a * v2.y() - b * v2.z();
    let (min, max) = if p0 < p2 { (p0, p2) } else { (p2, p0) };
    let rad = fa * h.y() + fb * h.z();
    !(min > rad || max < -rad)
}

fn axistest_x2(a: f64, b: f64, fa: f64, fb: f64, v0: Vector3, v1: Vector3, h: Vector3) -> bool {
    let p0 = a * v0.y() - b * v0.z();
    let p1 = a * v1.y() - b * v1.z();
    let (min, max) = if p0 < p1 { (p0, p1) } else { (p1, p0) };
    let rad = fa * h.y() + fb * h.z();
    !(min > rad || max < -rad)
}

fn axistest_y02(a: f64, b: f64, fa: f64, fb: f64, v0: Vector3, v2: Vector3, h: Vector3) -> bool {
    let p0 = -a * v0.x() + b * v0.z();
    let p2 = -a * v2.x() + b * v2.z();
    let (min, max) = if p0 < p2 { (p0, p2) } else { (p2, p0) };
    let rad = fa * h.x() + fb * h.z();
    !(min > rad || max < -rad)
}

fn axistest_y1(a: f64, b: f64, fa: f64, fb: f64, v0: Vector3, v1: Vector3, h: Vector3) -> bool {
    let p0 = -a * v0.x() + b * v0.z();
    let p1 = -a * v1.x() + b * v1.z();
    let (min, max) = if p0 < p1 { (p0, p1) } else { (p1, p0) };
    let rad = fa * h.x() + fb * h.z();
    !(min > rad || max < -rad)
}

fn axistest_z12(a: f64, b: f64, fa: f64, fb: f64, v1: Vector3, v2: Vector3, h: Vector3) -> bool {
    let p1 = a * v1.x() - b * v1.y();
    let p2 = a * v2.x() - b * v2.y();
    let (min, max) = if p1 < p2 { (p1, p2) } else { (p2, p1) };
    let rad = fa * h.x() + fb * h.y();
    !(min > rad || max < -rad)
}

fn axistest_z0(a: f64, b: f64, fa: f64, fb: f64, v0: Vector3, v1: Vector3, h: Vector3) -> bool {
    let p0 = a * v0.x() - b * v0.y();
    let p1 = a * v1.x() - b * v1.y();
    let (min, max) = if p0 < p1 { (p0, p1) } else { (p1, p0) };
    let rad = fa * h.x() + fb * h.y();
    !(min > rad || max < -rad)
}

fn findminmax(x0: f64, x1: f64, x2: f64) -> (f64, f64) {
    let mut min = x0;
    let mut max = x0;

    if x1 < min {
        min = x1
    }

    if x1 > max {
        max = x1
    }

    if x2 < min {
        min = x2
    }

    if x2 > max {
        max = x2
    }

    (min, max)
}

fn plane_box_overlap(normal: Vector3, vert: Vector3, maxbox: Vector3) -> bool {
    let mut vmin = Vector3::zeros();
    let mut vmax = Vector3::zeros();

    for q in 0..3 {
        let v = vert[q];

        if normal[q] > 0. {
            vmin[q] = -maxbox[q] - v;
            vmax[q] = maxbox[q] - v;
        } else {
            vmin[q] = maxbox[q] - v;
            vmax[q] = -maxbox[q] - v;
        }
    }

    Vector3::dot(&normal, &vmin) <= 0. && Vector3::dot(&normal, &vmax) >= 0.
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_aabb() -> Aabb {
        let center = Vector3::new(0.5, 0.5, 0.5);
        let halfsize = Vector3::new(0.5, 0.5, 0.5);
        Aabb::new(center, halfsize)
    }

    #[test]
    fn test_aabb_triangle_ok_inside() {
        let aabb = get_aabb();
        let p = Vector3::new(0.1, 0.1, 0.1);
        let q = Vector3::new(0.1, 0.1, 0.3);
        let r = Vector3::new(0.1, 0.3, 0.1);
        let triangle = Triangle::new(p, q, r);

        let intersects = intersects_aabb_triangle(&aabb, &triangle);

        assert!(intersects);
    }

    #[test]
    fn test_aabb_triangle_fail_aabb() {
        let aabb = get_aabb();
        let p = Vector3::new(0., 0., 2.);
        let q = Vector3::new(1., 0., 2.);
        let r = Vector3::new(1., 1., 2.);
        let triangle = Triangle::new(p, q, r);

        let intersects = intersects_aabb_triangle(&aabb, &triangle);

        assert!(!intersects);
    }

    #[test]
    fn test_aabb_triangle_fail_plane() {
        let aabb = get_aabb();
        let p = Vector3::new(0.1, 1.1, 0.9);
        let q = Vector3::new(0.5, 0.8, 1.5);
        let r = Vector3::new(0.9, 1.1, 0.9);
        let triangle = Triangle::new(p, q, r);

        let intersects = intersects_aabb_triangle(&aabb, &triangle);

        assert!(!intersects);
    }

    #[test]
    fn test_aabb_triangle_fail_axis_e0_x01() {
        let aabb = get_aabb();
        let p = Vector3::new(0.5, 1.1, 0.9);
        let q = Vector3::new(0.5, 0.8, 1.5);
        let r = Vector3::new(0.5, 1.3, 1.2);
        let triangle = Triangle::new(p, q, r);

        let intersects = intersects_aabb_triangle(&aabb, &triangle);

        assert!(!intersects);
    }

    #[test]
    fn test_aabb_triangle_fail_axis_e0_y02() {
        let aabb = get_aabb();
        let p = Vector3::new(1.1, 0.5, 0.9);
        let q = Vector3::new(0.8, 0.5, 1.5);
        let r = Vector3::new(1.3, 0.5, 1.2);
        let triangle = Triangle::new(p, q, r);

        let intersects = intersects_aabb_triangle(&aabb, &triangle);

        assert!(!intersects);
    }

    #[test]
    fn test_aabb_triangle_fail_axis_e0_z12() {
        let aabb = get_aabb();
        let p = Vector3::new(1.1, 0.9, 0.5);
        let q = Vector3::new(0.8, 1.5, 0.5);
        let r = Vector3::new(1.3, 1.2, 0.5);
        let triangle = Triangle::new(p, q, r);

        let intersects = intersects_aabb_triangle(&aabb, &triangle);

        assert!(!intersects);
    }

    #[test]
    fn test_aabb_triangle_fail_axis_e1_x01() {
        let aabb = get_aabb();
        let p = Vector3::new(0.5, 1.3, 1.2);
        let q = Vector3::new(0.5, 1.1, 0.9);
        let r = Vector3::new(0.5, 0.8, 1.5);
        let triangle = Triangle::new(p, q, r);

        let intersects = intersects_aabb_triangle(&aabb, &triangle);

        assert!(!intersects);
    }

    #[test]
    fn test_aabb_triangle_fail_axis_e1_y02() {
        let aabb = get_aabb();
        let p = Vector3::new(1.3, 0.5, 1.2);
        let q = Vector3::new(1.1, 0.5, 0.9);
        let r = Vector3::new(0.8, 0.5, 1.5);
        let triangle = Triangle::new(p, q, r);

        let intersects = intersects_aabb_triangle(&aabb, &triangle);

        assert!(!intersects);
    }

    #[test]
    fn test_aabb_triangle_fail_axis_e1_z0() {
        let aabb = get_aabb();
        let p = Vector3::new(1.3, 1.2, 0.5);
        let q = Vector3::new(1.1, 0.9, 0.5);
        let r = Vector3::new(0.8, 1.5, 0.5);
        let triangle = Triangle::new(p, q, r);

        let intersects = intersects_aabb_triangle(&aabb, &triangle);

        assert!(!intersects);
    }

    #[test]
    fn test_aabb_triangle_fail_axis_e2_x2() {
        let aabb = get_aabb();
        let p = Vector3::new(0.5, 0.8, 1.5);
        let q = Vector3::new(0.5, 1.3, 1.2);
        let r = Vector3::new(0.5, 1.1, 0.9);
        let triangle = Triangle::new(p, q, r);

        let intersects = intersects_aabb_triangle(&aabb, &triangle);

        assert!(!intersects);
    }

    #[test]
    fn test_aabb_triangle_fail_axis_e2_y1() {
        let aabb = get_aabb();
        let p = Vector3::new(0.8, 0.5, 1.5);
        let q = Vector3::new(1.3, 0.5, 1.2);
        let r = Vector3::new(1.1, 0.5, 0.9);
        let triangle = Triangle::new(p, q, r);

        let intersects = intersects_aabb_triangle(&aabb, &triangle);

        assert!(!intersects);
    }

    #[test]
    fn test_aabb_triangle_fail_axis_e2_z12() {
        let aabb = get_aabb();
        let p = Vector3::new(0.8, 1.5, 0.5);
        let q = Vector3::new(1.3, 1.2, 0.5);
        let r = Vector3::new(1.1, 0.9, 0.5);
        let triangle = Triangle::new(p, q, r);

        let intersects = intersects_aabb_triangle(&aabb, &triangle);

        assert!(!intersects);
    }
}
