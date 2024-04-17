use crate::geometry::Vector3;

/// Axis-aligned bounding box in three-dimensional Cartesian space.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Aabb {
    center: Vector3,
    halfsize: Vector3,
}

impl Aabb {
    /// Construct an Aabb from its center and halfsize
    pub fn new(center: Vector3, halfsize: Vector3) -> Aabb {
        Aabb { center, halfsize }
    }

    /// Construct an Aabb from its min and max bounds
    pub fn from_bounds(min: Vector3, max: Vector3) -> Aabb {
        let center = (max + min) * 0.5;
        let halfsize = (max - min) * 0.5;
        Aabb::new(center, halfsize)
    }

    /// Get the center
    pub fn center(&self) -> Vector3 {
        self.center
    }

    /// Get the halfsize
    pub fn halfsize(&self) -> Vector3 {
        self.halfsize
    }
}
