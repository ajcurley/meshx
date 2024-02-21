package meshx

type IntersectsAABB interface {
	IntersectsAABB(AABB) bool
}

type IntersectsRay interface {
	IntersectsRay(Ray) bool
}

type IntersectsTriangle interface {
	IntersectsTriangle(Triangle) bool
}
