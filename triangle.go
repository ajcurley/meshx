package meshx

// Triangle in three-dimension Cartesian space.
type Triangle struct {
	P Vector
	Q Vector
	R Vector
}

// Construct a Triangle from its vertices.
func NewTriangle(p, q, r Vector) Triangle {
	return Triangle{p, q, r}
}

// Compute the area.
func (t Triangle) Area() float64 {
	u := t.Q.Sub(t.P)
	v := t.R.Sub(t.P)
	return u.Cross(v).Mag() * 0.5
}

// Compute the normal.
func (t Triangle) Normal() Vector {
	u := t.Q.Sub(t.P)
	v := t.R.Sub(t.P)
	return u.Cross(v)
}

// Compute the unit normal.
func (t Triangle) UnitNormal() Vector {
	return t.Normal().Unit()
}

// Implement the IntersectsAABB interface.
func (t Triangle) IntersectsAABB(query AABB) bool {
	v0 := t.P.Sub(query.Center)
	v1 := t.Q.Sub(query.Center)
	v2 := t.R.Sub(query.Center)

	// Bounding query test
	if min(v0[0], v1[0], v2[0]) > query.HalfSize[0] || max(v0[0], v1[0], v2[0]) < -query.HalfSize[0] {
		return false
	}

	if min(v0[1], v1[1], v2[1]) > query.HalfSize[1] || max(v0[1], v1[1], v2[1]) < -query.HalfSize[1] {
		return false
	}

	if min(v0[2], v1[2], v2[2]) > query.HalfSize[2] || max(v0[2], v1[2], v2[2]) < -query.HalfSize[2] {
		return false
	}

	e0 := v1.Sub(v0)
	e1 := v2.Sub(v1)
	e2 := v0.Sub(v2)

	var fe Vector
	var p0, p1, p2 float64
	var rad float64

	// e0 axis tests
	fe = e0.Abs()

	// e0 - axis test X01
	p0 = e0[2]*v0[1] - e0[1]*v0[2]
	p2 = e0[2]*v2[1] - e0[1]*v2[2]
	rad = fe[2]*query.HalfSize[1] + fe[1]*query.HalfSize[2]

	if min(p0, p2) > rad || max(p0, p2) < -rad {
		return false
	}

	// e0 - axis test Y02
	p0 = -e0[2]*v0[0] + e0[0]*v0[2]
	p2 = -e0[2]*v2[0] + e0[0]*v2[2]
	rad = fe[2]*query.HalfSize[0] + fe[0]*query.HalfSize[2]

	if min(p0, p2) > rad || max(p0, p2) < -rad {
		return false
	}

	// e0 - axis test Z12
	p1 = e0[1]*v1[0] - e0[0]*v1[1]
	p2 = e0[1]*v2[0] - e0[0]*v2[1]
	rad = fe[1]*query.HalfSize[0] + fe[0]*query.HalfSize[1]

	if min(p1, p2) > rad || max(p1, p2) < -rad {
		return false
	}

	// e1 axis tests
	fe = e1.Abs()

	// e1 - axis test X01
	p0 = e1[2]*v0[1] - e1[1]*v0[2]
	p2 = e1[2]*v2[1] - e1[1]*v2[2]
	rad = fe[2]*query.HalfSize[1] + fe[1]*query.HalfSize[2]

	if min(p0, p2) > rad || max(p0, p2) < -rad {
		return false
	}

	// e1 - axis test Y02
	p0 = -e1[2]*v0[0] + e1[0]*v0[2]
	p2 = -e1[2]*v2[0] + e1[0]*v2[2]
	rad = fe[2]*query.HalfSize[0] + fe[0]*query.HalfSize[2]

	if min(p0, p2) > rad || max(p0, p2) < -rad {
		return false
	}

	// e1 - axis test Z0
	p0 = e1[1]*v0[0] - e1[0]*v0[1]
	p1 = e1[1]*v1[0] - e1[0]*v1[1]
	rad = fe[1]*query.HalfSize[0] + fe[0]*query.HalfSize[1]

	if min(p0, p1) > rad || max(p0, p1) < -rad {
		return false
	}

	// e2 axis tests
	fe = e2.Abs()

	// e2 - axis test X2
	p0 = e2[2]*v0[1] - e2[1]*v0[2]
	p1 = e2[2]*v1[1] - e2[1]*v1[2]
	rad = fe[2]*query.HalfSize[1] + fe[1]*query.HalfSize[2]

	if min(p0, p1) > rad || max(p0, p1) < -rad {
		return false
	}

	// e2 - axis test Y1
	p0 = -e2[2]*v0[0] + e2[0]*v0[2]
	p1 = -e2[2]*v1[0] + e2[0]*v1[2]
	rad = fe[2]*query.HalfSize[0] + fe[0]*query.HalfSize[2]

	if min(p0, p1) > rad || max(p0, p1) < -rad {
		return false
	}

	// e2 - axis test Z12
	p1 = e2[1]*v1[0] - e2[0]*v1[1]
	p2 = e2[1]*v2[0] - e2[0]*v2[1]
	rad = fe[1]*query.HalfSize[0] + fe[0]*query.HalfSize[1]

	if min(p1, p2) > rad || max(p1, p2) < -rad {
		return false
	}

	// Plane test
	n := e0.Cross(e1)
	e := query.HalfSize.Dot(n.Abs())
	s := -n.Dot(v0)

	return s-e <= 0 && s+e >= 0
}

// Implement the IntersectsRay interface.
func (t Triangle) IntersectsRay(query Ray) bool {
	return query.IntersectsTriangle(t)
}
