package meshx

type IntersectsAABB interface {
	IntersectsAABB(AABB) bool
}

type IntersectsTriangle interface {
	IntersectsTriangle(Triangle) bool
}
