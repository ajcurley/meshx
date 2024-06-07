use crate::geometry::{Triangle, Vector3};

#[derive(Debug, Copy, Clone, Default)]
struct Interval {
    a: f64,
    b: f64,
    c: f64,
    x0: f64,
    x1: f64,
}

/// Check for a spatial intersection two Triangles
pub fn intersects_triangle_triangle(t1: &Triangle, t2: &Triangle) -> bool {
    const EPSILON: f64 = 1e-6;

    // Unpack the vertices to match the nomenclature in the article
    let (v0, v1, v2) = (t1.p(), t1.q(), t1.r());
    let (u0, u1, u2) = (t2.p(), t2.q(), t2.r());

    // Compute the plane equation of the triangle t1
    let n1 = t1.normal();
    let d1 = -Vector3::dot(&n1, &v0);

    // Compute the signed distances to the plane of triangle t1 for each of
    // the vertices in triangle t2
    let du0 = Vector3::dot(&n1, &u0) + d1;
    let du1 = Vector3::dot(&n1, &u1) + d1;
    let du2 = Vector3::dot(&n1, &u2) + d1;

    let du0 = if du0 < EPSILON { 0. } else { du0 };
    let du1 = if du1 < EPSILON { 0. } else { du1 };
    let du2 = if du2 < EPSILON { 0. } else { du2 };

    let du0du1 = du0 * du1;
    let du0du2 = du0 * du2;

    // If all signed distances share the same sign and are not equal to 0, then
    // all vertices of t2 are on the same side of the t1 plane so there is no
    // intersection between the triangles.
    if du0du1 > 0. && du0du2 > 0. {
        return false;
    }

    // Compute the plane equation of triangle t2
    let n2 = t2.normal();
    let d2 = -Vector3::dot(&n2, &u0);

    // Compute the signed distances to the plane fo triangle t2 for each of
    // the vertices in triangle t1.
    let dv0 = Vector3::dot(&n2, &v0) + d2;
    let dv1 = Vector3::dot(&n2, &v1) + d2;
    let dv2 = Vector3::dot(&n2, &v2) + d2;

    let dv0 = if dv0 < EPSILON { 0. } else { dv0 };
    let dv1 = if dv1 < EPSILON { 0. } else { dv1 };
    let dv2 = if dv2 < EPSILON { 0. } else { dv2 };

    let dv0dv1 = dv0 * dv1;
    let dv0dv2 = dv0 * dv2;

    // If all signed distances share the same sign and are not equal to 0, then
    // all vertices of t1 are on the same side of the t2 plane so there is no
    // intersection between the triangles.
    if dv0dv1 > 0. && dv0dv2 > 0. {
        return false;
    }

    // Compute the direction of the intersection line
    let d = Vector3::cross(&n1, &n2);

    // Compute and index to the largest component of d.
    let index = d.abs().argmax();

    // Simplified projection into L
    let vp = Vector3::new(v0[index], v1[index], v2[index]);
    let up = Vector3::new(u0[index], u1[index], u2[index]);

    // Compute the interval for triangle 1
    let (interval1, _coplanar) = compute_interval(vp, dv0, dv1, dv2, dv0dv1, dv0dv2);
    // FIXME: handle coplanar

    // Compute the interval for triangle 2
    let (interval2, _coplanar) = compute_interval(up, du0, du1, du2, du0du1, du0du2);
    // FIXME: handle coplanar

    // Compute the overlap between the two intervals
    let xx = interval1.x0 * interval1.x1;
    let yy = interval2.x0 * interval2.x1;
    let xxyy = xx * yy;

    let tmp = interval1.a * xxyy;
    let i10 = tmp + interval1.b * interval1.x1 * yy;
    let i11 = tmp + interval1.c * interval1.x0 * yy;
    let (i10, i11) = if i10 < i11 { (i10, i11) } else { (i11, i10) };

    let tmp = interval2.a * xxyy;
    let i20 = tmp + interval2.b * xx * interval2.x1;
    let i21 = tmp + interval2.c * xx * interval2.x0;
    let (i20, i21) = if i20 < i21 { (i20, i21) } else { (i21, i20) };

    if i11 < i20 || i21 < i10 {
        return false;
    }

    true
}

fn compute_interval(
    vv: Vector3,
    d0: f64,
    d1: f64,
    d2: f64,
    d0d1: f64,
    d0d2: f64,
) -> (Interval, bool) {
    let mut interval = Interval::default();
    let mut coplanar = false;

    if d0d1 > 0. {
        // Here we know that d0d2 <= 0; that is d0, d1 are on the same side and d2
        // on the other side or on the plane.
        interval.a = vv.z();
        interval.b = (vv.x() - vv.z()) * d2;
        interval.c = (vv.y() - vv.z()) * d2;
        interval.x0 = d2 - d0;
        interval.x1 = d2 - d1;
    } else if d0d2 > 0. {
        // Here we know that d0d1 <= 0
        interval.a = vv.y();
        interval.b = (vv.x() - vv.y()) * d1;
        interval.c = (vv.z() - vv.y()) * d1;
        interval.x0 = d1 - d0;
        interval.x1 = d1 - d2;
    } else if d1 * d2 > 0. || d0 != 0. {
        // Here we know that d0d1 <= 0 or that d0 != 0
        interval.a = vv.x();
        interval.b = (vv.y() - vv.x()) * d0;
        interval.c = (vv.z() - vv.x()) * d0;
        interval.x0 = d0 - d1;
        interval.x1 = d0 - d2;
    } else if d1 != 0. {
        interval.a = vv.y();
        interval.b = (vv.x() - vv.y()) * d1;
        interval.c = (vv.z() - vv.y()) * d1;
        interval.x0 = d1 - d0;
        interval.x1 = d1 - d2;
    } else if d2 != 0. {
        interval.a = vv.z();
        interval.b = (vv.x() - vv.z()) * d2;
        interval.c = (vv.y() - vv.z()) * d2;
        interval.x0 = d2 - d0;
        interval.x1 = d2 - d1;
    } else {
        // Triangles are coplanar
        coplanar = true;
    }

    (interval, coplanar)
}
