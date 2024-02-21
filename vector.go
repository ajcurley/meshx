package meshx

import (
	"math"
)

// Cartesian vector in three-dimensional space.
type Vector [3]float64

// Get the X component.
func (v Vector) X() float64 {
	return v[0]
}

// Get the Y component.
func (v Vector) Y() float64 {
	return v[1]
}

// Get the Z component.
func (v Vector) Z() float64 {
	return v[2]
}

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
		v.X() / mag,
		v.Y() / mag,
		v.Z() / mag,
	}
}

// Add two vectors v + w.
func (v Vector) Add(w Vector) Vector {
	return Vector{
		v.X() + w.X(),
		v.Y() + w.Y(),
		v.Z() + w.Z(),
	}
}

// Add a scalar to a vector.
func (v Vector) AddScalar(s float64) Vector {
	return Vector{
		v.X() + s,
		v.Y() + s,
		v.Z() + s,
	}
}

// Subtract two vectors v - w.
func (v Vector) Sub(w Vector) Vector {
	return Vector{
		v.X() - w.X(),
		v.Y() - w.Y(),
		v.Z() - w.Z(),
	}
}

// Subtract a scalar from a vector.
func (v Vector) SubScalar(s float64) Vector {
	return Vector{
		v.X() - s,
		v.Y() - s,
		v.Z() - s,
	}
}

// Multiply two vectors element-wise v * w.
func (v Vector) Mul(w Vector) Vector {
	return Vector{
		v.X() * w.X(),
		v.Y() * w.Y(),
		v.Z() * w.Z(),
	}
}

// Multiply a scalar by a vector.
func (v Vector) MulScalar(s float64) Vector {
	return Vector{
		v.X() * s,
		v.Y() * s,
		v.Z() * s,
	}
}

// Divide two vectors element-wise v / w.
func (v Vector) Div(w Vector) Vector {
	return Vector{
		v.X() / w.X(),
		v.Y() / w.Y(),
		v.Z() / w.Z(),
	}
}

// Divide a vector by a scalar.
func (v Vector) DivScalar(s float64) Vector {
	return Vector{
		v.X() / s,
		v.Y() / s,
		v.Z() / s,
	}
}

// Compute the dot product v * w.
func (v Vector) Dot(w Vector) float64 {
	return v.X()*w.X() + v.Y()*w.Y() + v.Z()*w.Z()
}

// Compute the cross product v x w.
func (v Vector) Cross(w Vector) Vector {
	return Vector{
		v.Y()*w.Z() - v.Z()*w.Y(),
		v.Z()*w.X() - v.X()*w.Z(),
		v.X()*w.Y() - v.Y()*w.X(),
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
