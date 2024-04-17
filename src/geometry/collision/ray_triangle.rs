use crate::geometry::{Ray, Triangle, Vector3, EPSILON};

/// Check if the Ray/Triangle intersect
pub fn intersects_ray_triangle(ray: &Ray, triangle: &Triangle) -> bool {
    let e1 = triangle[1] - triangle[0];
    let e2 = triangle[2] - triangle[0];
    let direction = ray.direction();
    let origin = ray.origin();

    let p = Vector3::cross(&direction, &e2);
    let d = Vector3::dot(&e1, &p);

    if d < EPSILON {
        return false;
    }

    let d_inv = 1. / d;
    let s = origin - triangle[0];
    let u = d_inv * Vector3::dot(&s, &p);

    if u < 0. || u > 1. {
        return false;
    }

    let q = Vector3::cross(&s, &e1);
    let v = d_inv * Vector3::dot(&direction, &q);

    if v < 0. || u + v > 1. {
        return false;
    }

    d_inv * Vector3::dot(&e2, &q) > EPSILON
}
