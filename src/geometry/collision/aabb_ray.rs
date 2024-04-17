use crate::geometry::{Aabb, Ray};

/// Check if the Aabb and Ray intersect
pub fn intersects_aabb_ray(aabb: &Aabb, ray: &Ray) -> bool {
    let min = aabb.min();
    let max = aabb.max();
    let inv = ray.direction().inv();
    let origin = ray.origin();

    let mut tmin = std::f64::NEG_INFINITY;
    let mut tmax = std::f64::INFINITY;

    for i in 0..3 {
        let t1 = (min[i] - origin[i]) * inv[i];
        let t2 = (max[i] - origin[i]) * inv[i];
        tmin = tmin.max(t1.min(t2));
        tmax = tmax.min(t1.min(t2));
    }

    tmax >= tmin.max(0.)
}
