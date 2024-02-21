package meshx

import (
//"math"
)

// Ray in three-dimensional Cartesian space.
type Ray struct {
	Origin    Vector
	Direction Vector
}

// Construct a Ray from its origin and direction.
func NewRay(origin, direction Vector) Ray {
	return Ray{origin, direction}
}

// Implement the IntersectsAABB interface.
func (r Ray) IntersectsAABB(query AABB) bool {
	var tmin, tmax, t1, t2 float64
	minBound := query.GetMinBound()
	maxBound := query.GetMaxBound()

	t1 = (minBound[0] - r.Origin[0]) / r.Direction[0]
	t2 = (maxBound[0] - r.Origin[0]) / r.Direction[0]
	tmin = min(t1, t2)
	tmax = max(t1, t2)

	t1 = (minBound[1] - r.Origin[1]) / r.Direction[1]
	t2 = (maxBound[1] - r.Origin[1]) / r.Direction[1]
	tmin = max(tmin, min(t1, t2))
	tmax = min(tmax, max(t1, t2))

	t1 = (minBound[2] - r.Origin[2]) / r.Direction[2]
	t2 = (maxBound[2] - r.Origin[2]) / r.Direction[2]
	tmin = max(tmin, min(t1, t2))
	tmax = min(tmax, max(t1, t2))

	return tmax >= max(tmin, 0)
}

// Implement the IntersectsTriangle interface.
func (r Ray) IntersectsTriangle(query Triangle) bool {
	const epsilon float64 = 1e-7

	v0v1 := query.Q.Sub(query.P)
	v0v2 := query.R.Sub(query.P)

	p := r.Direction.Cross(v0v2)
	d := v0v1.Dot(p)

	if d < epsilon {
		return false
	}

	inv := 1 / d
	t := r.Origin.Sub(query.P)
	u := t.Dot(p) * inv

	if u < 0 || u > 1 {
		return false
	}

	q := t.Cross(v0v1)
	v := r.Direction.Dot(q) * inv

	return v >= 0 && u+v <= 1
}
