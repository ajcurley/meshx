package meshx

type IntersectsAABB interface {
	IntersectsAABB(AABB) bool
}

type IntersectsSphere interface {
	IntersectsSphere(Sphere) bool
}

type IntersectsTriangle interface {
	IntersectsTriangle(Triangle) bool
}
