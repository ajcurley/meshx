use crate::geometry::{Line, Plane, Vector3, EPSILON};

/// Compute the intersection point between a Line and a Plane
pub fn intersection_line_plane(line: &Line, plane: &Plane) -> Option<Vector3> {
    let normal = plane.normal();
    let u = line.q() - line.p();
    let dot = Vector3::dot(&normal, &u);

    if dot.abs() > EPSILON {
        let c = normal * -plane.d() / Vector3::dot(&normal, &normal);
        let w = line.p() - c;
        return Some(line.p() + u * -Vector3::dot(&normal, &w) / dot);
    }

    None
}
