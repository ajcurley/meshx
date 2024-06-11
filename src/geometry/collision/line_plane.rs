use crate::geometry::{Distance, Line, Plane, Vector3};

/// Compute the intersection point between a Line and a Plane
pub fn intersection_line_plane(line: &Line, plane: &Plane) -> Option<Vector3> {
    let r = line.q() - line.p();
    Some(line.p() + r * (-plane.distance(&line.p()) / Vector3::dot(&plane.normal(), &r)))
}
