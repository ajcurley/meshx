use crate::geometry::{Sphere, Vector3};

/// Check if the Sphere/Sphere intersect
pub fn intersects_sphere_sphere(a: &Sphere, b: &Sphere) -> bool {
    let d = a.center() - b.center();
    let r = a.radius() + b.radius();
    Vector3::dot(&d, &d) <= r * r
}
