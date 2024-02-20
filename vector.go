package meshx

import (
	"math"
)

// Cartesian vector in three-dimensional space.
type Vector struct {
	X float64
	Y float64
	Z float64
}

// Compute the magnitude (L2-norm).
func (v Vector) Mag() float64 {
	return math.Sqrt(v.Dot(v))
}

// Compute the unit.
func (v Vector) Unit() Vector {
	mag := v.Mag()
	return Vector{
		X: v.X / mag,
		Y: v.Y / mag,
		Z: v.Z / mag,
	}
}

// Add two vectors v + w.
func (v Vector) Add(w Vector) Vector {
	return Vector{
		X: v.X + w.X,
		Y: v.Y + w.Y,
		Z: v.Z + w.Z,
	}
}

// Add a scalar to a vector.
func (v Vector) AddScalar(s float64) Vector {
	return Vector{
		X: v.X + s,
		Y: v.Y + s,
		Z: v.Z + s,
	}
}

// Subtract two vectors v - w.
func (v Vector) Sub(w Vector) Vector {
	return Vector{
		X: v.X - w.X,
		Y: v.Y - w.Y,
		Z: v.Z - w.Z,
	}
}

// Subtract a scalar from a vector.
func (v Vector) SubScalar(s float64) Vector {
	return Vector{
		X: v.X - s,
		Y: v.Y - s,
		Z: v.Z - s,
	}
}

// Multiply two vectors element-wise v * w.
func (v Vector) Mul(w Vector) Vector {
	return Vector{
		X: v.X * w.X,
		Y: v.Y * w.Y,
		Z: v.Z * w.Z,
	}
}

// Multiply a scalar by a vector.
func (v Vector) MulScalar(s float64) Vector {
	return Vector{
		X: v.X * s,
		Y: v.Y * s,
		Z: v.Z * s,
	}
}

// Divide two vectors element-wise v / w.
func (v Vector) Div(w Vector) Vector {
	return Vector{
		X: v.X / w.X,
		Y: v.Y / w.Y,
		Z: v.Z / w.Z,
	}
}

// Divide a vector by a scalar.
func (v Vector) DivScalar(s float64) Vector {
	return Vector{
		X: v.X / s,
		Y: v.Y / s,
		Z: v.Z / s,
	}
}

// Compute the dot product v * w.
func (v Vector) Dot(w Vector) float64 {
	return v.X*w.X + v.Y*w.Y + v.Z*w.Z
}

// Compute the cross product v x w.
func (v Vector) Cross(w Vector) Vector {
	return Vector{
		X: v.Y*w.Z - v.Z*w.Y,
		Y: v.Z*w.X - v.X*w.Z,
		Z: v.X*w.Y - v.Y*w.X,
	}
}
