use crate::geometry::Vector3;

#[derive(Debug, Copy, Clone)]
pub struct Plane {
    normal: Vector3,
    d: f64,
}

impl Plane {
    /// Construct a Plane from its normal and constant
    pub fn new(normal: Vector3, d: f64) -> Plane {
        Plane { normal, d }
    }

    /// Construct a Plane from three points
    pub fn from_points(p: Vector3, q: Vector3, r: Vector3) -> Plane {
        let u = q - p;
        let v = r - p;
        let normal = Vector3::cross(&u, &v).unit();
        let d = -Vector3::dot(&normal, &p);
        Plane::new(normal, d)
    }

    /// Get the normal
    pub fn normal(&self) -> Vector3 {
        self.normal
    }

    /// Get the constant
    pub fn d(&self) -> f64 {
        self.d
    }
}
