package halfedge

import (
	"github.com/ajcurley/meshx"
)

type Vertex struct {
	Point    meshx.Vector
	HalfEdge int
}
