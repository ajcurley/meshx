use crate::geometry::{Plane, Vector3};

/// Compute the distance from a point to a Plane
pub fn distance_plane_vector3(plane: &Plane, v: &Vector3) -> f64 {
    Vector3::dot(&plane.normal(), v) + plane.d()
}
