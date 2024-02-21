package meshx

// Axis aligned bounding box.
type AABB struct {
	Center   Vector
	HalfSize Vector
}

// Construct an AABB from its center and halfsize.
func NewAABB(center, halfSize Vector) AABB {
	return AABB{center, halfSize}
}

// Construct an AABB from its min/max bounds.
func NewAABBFromBounds(minBound, maxBound Vector) AABB {
	center := maxBound.Add(minBound).MulScalar(0.5)
	halfSize := maxBound.Sub(minBound).MulScalar(0.5)
	return NewAABB(center, halfSize)
}

// Construct an AABB from a slice of vectors.
func NewAABBFromVectors(vectors []Vector) AABB {
	minBound := vectors[0]
	maxBound := vectors[0]

	for _, vector := range vectors[1:] {
		for i := 0; i < 3; i++ {
			if vector[i] < minBound[i] {
				minBound[i] = vector[i]
			}

			if vector[i] > maxBound[i] {
				maxBound[i] = vector[i]
			}
		}
	}

	return NewAABBFromBounds(minBound, maxBound)
}

// Construct an AABB with a buffer (percentage of the edge length).
func (a AABB) Buffer(s float64) AABB {
	return NewAABB(a.Center, a.HalfSize.MulScalar(1+s))
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
		center[0] += halfSize[0]
	} else {
		center[0] -= halfSize[0]
	}

	if octant&2 == 2 {
		center[1] += halfSize[1]
	} else {
		center[1] -= halfSize[1]
	}

	if octant&1 == 1 {
		center[2] += halfSize[2]
	} else {
		center[2] -= halfSize[2]
	}

	return AABB{center, halfSize}
}

// Implement the IntersectsAABB interface.
func (a AABB) IntersectsAABB(query AABB) bool {
	aMin := a.GetMinBound()
	aMax := a.GetMaxBound()
	qMin := query.GetMinBound()
	qMax := query.GetMaxBound()

	return aMin[0] <= qMax[0] &&
		aMax[0] >= qMin[0] &&
		aMin[1] <= qMax[1] &&
		aMax[1] >= qMin[1] &&
		aMin[2] <= qMax[2] &&
		aMax[2] >= qMin[2]
}
