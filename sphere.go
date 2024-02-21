package meshx

// Sphere in three-dimensional Cartesian space.
type Sphere struct {
	Center Vector
	Radius float64
}

// Construct a Sphere from its center and radius.
func NewSphere(center Vector, radius float64) Sphere {
	return Sphere{center, radius}
}

// Implement the IntersectsAABB interface.
func (s Sphere) IntersectsAABB(query AABB) bool {
	panic("not implemented")
}
