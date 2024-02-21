package meshx

// Axis aligned bounding box.
type AABB struct {
	Center   Vector
	HalfSize Vector
}

// Get the minimum bound.
func (a AABB) GetMinBound() Vector {
	return a.Center.Sub(a.HalfSize)
}

// Get the maximum bound.
func (a AABB) GetMaxBound() Vector {
	return a.Center.Add(a.HalfSize)
}

// Compute the octant AABB.
func (a AABB) Octant(octant int) AABB {
	if octant < 0 || octant >= 8 {
		panic("octant out of range")
	}

	halfSize := a.HalfSize.MulScalar(0.5)
	center := a.Center

	if octant&4 == 4 {
		center.X += halfSize.X
	} else {
		center.X -= halfSize.X
	}

	if octant&2 == 2 {
		center.Y += halfSize.Y
	} else {
		center.Y -= halfSize.Y
	}

	if octant&1 == 1 {
		center.Z += halfSize.Z
	} else {
		center.Z -= halfSize.Z
	}

	return AABB{center, halfSize}
}

// Implement the IntersectsAABB interface.
func (a AABB) IntersectsAABB(query AABB) bool {
	aMin := a.GetMinBound()
	aMax := a.GetMaxBound()
	qMin := query.GetMinBound()
	qMax := query.GetMaxBound()

	return aMin.X <= qMax.X &&
		aMax.X >= qMin.X &&
		aMin.Y <= qMax.Y &&
		aMax.Y >= qMin.Y &&
		aMin.Z <= qMax.Z &&
		aMax.Z >= qMin.Z
}
