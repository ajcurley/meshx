package meshx

import (
	"math"
)

// Cartesian vector in three-dimensional space.
type Vector [3]float64

// Construct a Vector from its components.
func NewVector(x, y, z float64) Vector {
	return Vector{x, y, z}
}

// Construct a Vector from an array.
func NewVectorFromArray(values [3]float64) Vector {
	return Vector(values)
}

// Compute the magnitude (L2-norm).
func (v Vector) Mag() float64 {
	return math.Sqrt(v.Dot(v))
}

// Compute the unit.
func (v Vector) Unit() Vector {
	mag := v.Mag()
	return Vector{
		v[0] / mag,
		v[1] / mag,
		v[2] / mag,
	}
}

// Compute the absolute vector.
func (v Vector) Abs() Vector {
	return Vector{
		math.Abs(v[0]),
		math.Abs(v[1]),
		math.Abs(v[2]),
	}
}

// Add two vectors v + w.
func (v Vector) Add(w Vector) Vector {
	return Vector{
		v[0] + w[0],
		v[1] + w[1],
		v[2] + w[2],
	}
}

// Add a scalar to a vector.
func (v Vector) AddScalar(s float64) Vector {
	return Vector{
		v[0] + s,
		v[1] + s,
		v[2] + s,
	}
}

// Subtract two vectors v - w.
func (v Vector) Sub(w Vector) Vector {
	return Vector{
		v[0] - w[0],
		v[1] - w[1],
		v[2] - w[2],
	}
}

// Subtract a scalar from a vector.
func (v Vector) SubScalar(s float64) Vector {
	return Vector{
		v[0] - s,
		v[1] - s,
		v[2] - s,
	}
}

// Multiply two vectors element-wise v * w.
func (v Vector) Mul(w Vector) Vector {
	return Vector{
		v[0] * w[0],
		v[1] * w[1],
		v[2] * w[2],
	}
}

// Multiply a scalar by a vector.
func (v Vector) MulScalar(s float64) Vector {
	return Vector{
		v[0] * s,
		v[1] * s,
		v[2] * s,
	}
}

// Divide two vectors element-wise v / w.
func (v Vector) Div(w Vector) Vector {
	return Vector{
		v[0] / w[0],
		v[1] / w[1],
		v[2] / w[2],
	}
}

// Divide a vector by a scalar.
func (v Vector) DivScalar(s float64) Vector {
	return Vector{
		v[0] / s,
		v[1] / s,
		v[2] / s,
	}
}

// Compute the dot product v * w.
func (v Vector) Dot(w Vector) float64 {
	return v[0]*w[0] + v[1]*w[1] + v[2]*w[2]
}

// Compute the cross product v x w.
func (v Vector) Cross(w Vector) Vector {
	return Vector{
		v[1]*w[2] - v[2]*w[1],
		v[2]*w[0] - v[0]*w[2],
		v[0]*w[1] - v[1]*w[0],
	}
}

// Implement the IntersectsAABB interface.
func (v Vector) IntersectsAABB(query AABB) bool {
	for i := 0; i < 3; i++ {
		if v[i] < query.Center[i]-query.HalfSize[i] {
			return false
		}

		if v[i] > query.Center[i]+query.HalfSize[i] {
			return false
		}
	}

	return true
}

// Impement the IntersectsSphere interface.
func (v Vector) IntersectsSphere(query Sphere) bool {
	return query.Center.Sub(v).Mag() <= query.Radius * query.Radius
}
