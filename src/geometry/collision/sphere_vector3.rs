use crate::geometry::{Sphere, Vector3};

/// Check if the Sphere/Vector3 intersect
pub fn intersects_sphere_vector3(sphere: &Sphere, v: &Vector3) -> bool {
    (*v - sphere.center()).mag() <= sphere.radius()
}
