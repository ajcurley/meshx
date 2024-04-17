use crate::geometry::Aabb;

/// Check if the Aabbs intersect
pub fn intersects_aabb_aabb(a: &Aabb, b: &Aabb) -> bool {
    let a_min = a.min();
    let a_max = a.max();
    let b_min = b.min();
    let b_max = b.max();

    a_min.x() <= b_max.x()
        && a_max.x() >= b_min.x()
        && a_min.y() <= b_max.y()
        && a_max.y() >= b_min.y()
        && a_min.z() <= b_max.z()
        && a_max.z() >= b_min.z()
}
