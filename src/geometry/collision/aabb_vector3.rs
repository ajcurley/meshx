use crate::geometry::{Aabb, Vector3, EPSILON};

/// Check if the Vector3 intersects the Aabb
pub fn intersects_aabb_vector3(aabb: &Aabb, v: &Vector3) -> bool {
    let min = aabb.min();
    let max = aabb.max();

    v.x() >= min.x() - EPSILON
        && v.x() <= max.x() + EPSILON
        && v.y() >= min.y() - EPSILON
        && v.y() <= max.y() + EPSILON
        && v.z() >= min.z() - EPSILON
        && v.z() <= max.z() + EPSILON
}
