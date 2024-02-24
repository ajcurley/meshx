package meshx

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

// Test a triangle area computation.
func TestTriangleArea(t *testing.T) {
	triangle := Triangle{
		P: NewVector(0, 0, 0),
		Q: NewVector(1, 0, 0),
		R: NewVector(1, 1, 0),
	}

	assert.Equal(t, 0.5, triangle.Area())
}

// Test a triangle normal computation.
func TestTriangleNormal(t *testing.T) {
	triangle := Triangle{
		P: NewVector(0, 0, 0),
		Q: NewVector(1, 0, 0),
		R: NewVector(1, 2, 0),
	}

	normal := triangle.Normal()
	assert.Equal(t, 0.0, normal[0])
	assert.Equal(t, 0.0, normal[1])
	assert.Equal(t, 2.0, normal[2])
}

// Test a triangle unit normal computation.
func TestTriangleUnitNormal(t *testing.T) {
	triangle := Triangle{
		P: NewVector(0, 0, 0),
		Q: NewVector(1, 0, 0),
		R: NewVector(1, 2, 0),
	}

	normal := triangle.UnitNormal()
	assert.Equal(t, 0.0, normal[0])
	assert.Equal(t, 0.0, normal[1])
	assert.Equal(t, 1.0, normal[2])
}

// Test a triangle/AABB intersection fully inside.
func TestTriangleIntersectsAABBInside(t *testing.T) {
	aabb := AABB{
		Center:   NewVector(0.5, 0.5, 0.5),
		HalfSize: NewVector(0.5, 0.5, 0.5),
	}

	triangle := Triangle{
		P: NewVector(0.25, 0.25, 0.25),
		Q: NewVector(0.25, 0.75, 0.25),
		R: NewVector(0.75, 0.75, 0.75),
	}

	assert.True(t, triangle.IntersectsAABB(aabb))
}

// Test a triangle/AABB intersection crossing a face plane.
func TestTriangleIntersectsAABBCrossFace(t *testing.T) {
	aabb := AABB{
		Center:   NewVector(0.5, 0.5, 0.5),
		HalfSize: NewVector(0.5, 0.5, 0.5),
	}

	triangle := Triangle{
		P: NewVector(0.5, 0.5, 0.5),
		Q: NewVector(2.0, -1.0, 0.5),
		R: NewVector(2.0, 1.0, 0.5),
	}

	assert.True(t, triangle.IntersectsAABB(aabb))
}

// Test a triangle/AABB intersection miss/outside.
func TestTriangleIntersectsAABBOutside(t *testing.T) {
	aabb := AABB{
		Center:   NewVector(0.5, 0.5, 0.5),
		HalfSize: NewVector(0.5, 0.5, 0.5),
	}

	triangle := Triangle{
		P: NewVector(1.25, 0.25, 0.25),
		Q: NewVector(1.25, 0.75, 0.25),
		R: NewVector(1.75, 0.75, 0.75),
	}

	assert.False(t, triangle.IntersectsAABB(aabb))
}

// Test a triangle/ray intersection.
func TestTriangleIntersectsRayHit(t *testing.T) {
	ray := Ray{
		Origin:    NewVector(0.5, 0.5, 0),
		Direction: NewVector(0, 0, 1),
	}

	triangle := Triangle{
		P: NewVector(0, 0, 2),
		Q: NewVector(0, 1, 2),
		R: NewVector(1, 1, 2),
	}

	assert.True(t, triangle.IntersectsRay(ray))
}

// Test a triangle/ray intersection hit culled for orientation.
func TestTriangleIntersectsRayHitCulled(t *testing.T) {
	ray := Ray{
		Origin:    NewVector(0.5, 0.5, 0),
		Direction: NewVector(0, 0, 1),
	}

	triangle := Triangle{
		P: NewVector(0, 0, 2),
		Q: NewVector(1, 0, 2),
		R: NewVector(1, 1, 2),
	}

	assert.False(t, triangle.IntersectsRay(ray))
}

// Test a triangle/ray intersection miss.
func TestTriangleIntersectsRayMiss(t *testing.T) {
	ray := Ray{
		Origin:    NewVector(0.5, 0.5, 0),
		Direction: NewVector(0, 0, -1),
	}

	triangle := Triangle{
		P: NewVector(0, 0, 2),
		Q: NewVector(1, 0, 2),
		R: NewVector(1, 1, 2),
	}

	assert.False(t, triangle.IntersectsRay(ray))
}

// Test a triangle/ray intersection miss beside.
func TestTriangleIntersectsRayBesideMiss(t *testing.T) {
	ray := Ray{
		Origin:    NewVector(1.5, 1.5, 0),
		Direction: NewVector(0, 0, 1),
	}

	triangle := Triangle{
		P: NewVector(0, 0, 2),
		Q: NewVector(0, 1, 2),
		R: NewVector(1, 1, 2),
	}

	assert.False(t, triangle.IntersectsRay(ray))
}
