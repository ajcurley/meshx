use crate::geometry::{Ray, Sphere, Vector3};

/// Check if the Ray/Sphere intersect
pub fn intersects_ray_sphere(ray: &Ray, sphere: &Sphere) -> bool {
    let u = sphere.center() - ray.origin();
    let v = ray.direction().unit();
    let r = sphere.radius();

    r * r - (Vector3::dot(&u, &u) - Vector3::dot(&u, &v)) >= 0.
}
