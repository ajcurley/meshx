package meshx

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

// Test a ray/AABB intersection with the ray originating inside.
func TestRayIntersectsAABBOriginInside(t *testing.T) {
	aabb := AABB{
		Center:   NewVector(0.5, 0.5, 0.5),
		HalfSize: NewVector(0.5, 0.5, 0.5),
	}

	ray := Ray{
		Origin:    NewVector(0.5, 0.5, 0.5),
		Direction: NewVector(1, 0, 0),
	}

	assert.True(t, ray.IntersectsAABB(aabb))
}

// Test a ray/AABB intersection with the ray originating outside.
func TestRayIntersectsAABBOriginOutside(t *testing.T) {
	aabb := AABB{
		Center:   NewVector(0.5, 0.5, 0.5),
		HalfSize: NewVector(0.5, 0.5, 0.5),
	}

	ray := Ray{
		Origin:    NewVector(-10, 0.5, 0.5),
		Direction: NewVector(1, 0, 0),
	}

	assert.True(t, ray.IntersectsAABB(aabb))
}

// Test a ray/AABB intersection with the ray along the X-edge of the
// AABB. This is an edge case that currently returns no hit.
func TestRayIntersectsAABBAlongX(t *testing.T) {
	aabb := AABB{
		Center:   NewVector(0.5, 0.5, 0.5),
		HalfSize: NewVector(0.5, 0.5, 0.5),
	}

	ray := Ray{
		Origin:    NewVector(-1, 0, 0),
		Direction: NewVector(1, 0, 0),
	}

	assert.False(t, ray.IntersectsAABB(aabb))
}

// Test a ray/AABB intersection with the ray along the Y-edge of the
// AABB. This is an edge case that currently returns no hit.
func TestRayIntersectsAABBAlongY(t *testing.T) {
	aabb := AABB{
		Center:   NewVector(0.5, 0.5, 0.5),
		HalfSize: NewVector(0.5, 0.5, 0.5),
	}

	ray := Ray{
		Origin:    NewVector(0, -1, 0),
		Direction: NewVector(0, 1, 0),
	}

	assert.False(t, ray.IntersectsAABB(aabb))
}

// Test a ray/AABB intersection with the ray along the Z-edge of the
// AABB. This is an edge case that currently returns no hit.
func TestRayIntersectsAABBAlongZ(t *testing.T) {
	aabb := AABB{
		Center:   NewVector(0.5, 0.5, 0.5),
		HalfSize: NewVector(0.5, 0.5, 0.5),
	}

	ray := Ray{
		Origin:    NewVector(0, 0, -1),
		Direction: NewVector(0, 0, 1),
	}

	assert.False(t, ray.IntersectsAABB(aabb))
}

// Test a ray/AABB intersection miss reverse direction.
func TestRayIntersectsAABBMissDirection(t *testing.T) {
	aabb := AABB{
		Center:   NewVector(0.5, 0.5, 0.5),
		HalfSize: NewVector(0.5, 0.5, 0.5),
	}

	ray := Ray{
		Origin:    NewVector(-1, 0.5, 0.5),
		Direction: NewVector(-1, 0, 0),
	}

	assert.False(t, ray.IntersectsAABB(aabb))
}

// Test a ray/AABB intersection miss beside the AABB.
func TestRayIntersectsAABBMissBeside(t *testing.T) {
	aabb := AABB{
		Center:   NewVector(0.5, 0.5, 0.5),
		HalfSize: NewVector(0.5, 0.5, 0.5),
	}

	ray := Ray{
		Origin:    NewVector(-1, 0, 2),
		Direction: NewVector(1, 0, 0),
	}

	assert.False(t, ray.IntersectsAABB(aabb))
}
