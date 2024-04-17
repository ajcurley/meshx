use crate::geometry::{Aabb, Vector3};

/// Check if the Vector3 intersects the Aabb
pub fn intersects_aabb_vector3(aabb: &Aabb, v: &Vector3) -> bool {
    let min = aabb.min();
    let max = aabb.max();

    v.x() >= min.x()
        && v.x() <= max.x()
        && v.y() >= min.y()
        && v.y() <= max.y()
        && v.z() >= min.z()
        && v.z() <= max.z()
}
