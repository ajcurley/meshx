use crate::geometry::{Aabb, Sphere};

/// Check if the Aabb/Sphere intersect
pub fn intersects_aabb_sphere(aabb: &Aabb, sphere: &Sphere) -> bool {
    let center = sphere.center();
    let min = aabb.min();
    let max = aabb.max();
    let mut d = 0.;

    for i in 0..3 {
        if center[i] < min[i] {
            let s = center[i] - min[i];
            d += s * s;
        } else if center[i] > max[i] {
            let s = center[i] - max[i];
            d += s * s;
        }
    }

    d <= sphere.radius() * sphere.radius()
}
