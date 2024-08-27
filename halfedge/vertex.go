package halfedge

import (
	"github.com/ajcurley/meshx-go"
)

type Vertex struct {
	Point    meshx.Vector
	HalfEdge int
}
