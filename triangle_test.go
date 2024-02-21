package meshx

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestTriangleIntersectsAABBInside(t *testing.T) {
	aabb := AABB{
		Center:   Vector{-2.4185921203884364, -0.3416792757025359, 0.17053477109453638},
		HalfSize: Vector{0.08006801913818454, 0.031034698874037734, 0.022564904556919972},
	}

	triangle := Triangle{
		P: Vector{-2.370567000215881, -0.3519710427659959, 0.20187195586823056},
		Q: Vector{-2.3682044632713737, -0.35553508857840443, 0.19207826859408414},
		R: Vector{-2.3676851697971846, -0.36355834988094166, 0.20201277596776676},
	}

	assert.True(t, triangle.IntersectsAABB(aabb))
}

func TestTriangleIntersectsAABBCrosses(t *testing.T) {
	aabb := AABB{
		Center:   Vector{0.5, 0.5, 0.5},
		HalfSize: Vector{0.5, 0.5, 0.5},
	}

	triangle := Triangle{
		P: Vector{1.143, 0.432, 0.274},
		Q: Vector{0.943, 0.774, 0.043},
		R: Vector{0.642, 0.333, 0.100},
	}

	assert.True(t, triangle.IntersectsAABB(aabb))
}

func TestTriangleIntersectsAABBX0(t *testing.T) {
	aabb := AABB{
		Center:   Vector{0.5, 0.5, 0.5},
		HalfSize: Vector{0.5, 0.5, 0.5},
	}

	triangle := Triangle{
		P: Vector{0.5, 0.5, 0.5},
		Q: Vector{0.5, -1, -1},
		R: Vector{0.5, -1, 0.5},
	}

	assert.True(t, triangle.IntersectsAABB(aabb))
}

func TestTriangleIntersectsAABBX1(t *testing.T) {
	aabb := AABB{
		Center:   Vector{0.5, 0.5, 0.5},
		HalfSize: Vector{0.5, 0.5, 0.5},
	}

	triangle := Triangle{
		P: Vector{0.5, 0.5, 0.5},
		Q: Vector{0.5, 1, -1},
		R: Vector{0.5, 1, 0.5},
	}

	assert.True(t, triangle.IntersectsAABB(aabb))
}

func TestTriangleIntersectsAABBOutside(t *testing.T) {
	aabb := AABB{
		Center:   Vector{0.5, 0.5, 0.5},
		HalfSize: Vector{0.5, 0.5, 0.5},
	}

	triangle := Triangle{
		P: Vector{1.143, 1.432, 1.274},
		Q: Vector{1.943, 1.774, 1.043},
		R: Vector{1.642, 1.333, 1.100},
	}

	assert.False(t, triangle.IntersectsAABB(aabb))
}
