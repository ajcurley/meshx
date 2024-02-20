package meshx

// Triangle in three-dimension Cartesian space.
type Triangle struct {
	P Vector
	Q Vector
	R Vector
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
