package spatial

import (
	"fmt"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"

	"github.com/ajcurley/meshx"
	"github.com/ajcurley/meshx/exchange"
)

func TestOctreeBuildVector(t *testing.T) {
	path := "/Users/acurley/projects/cfd/geometry/car.obj"
	reader, err := exchange.ReadOBJFromPath(path)
	assert.Empty(t, err)

	vertices := make([]meshx.Vector, reader.GetNumberOfVertices())

	for i := 0; i < reader.GetNumberOfVertices(); i++ {
		vertices[i] = reader.GetVertex(i)
	}

	start := time.Now()

	aabb := meshx.NewAABBFromVectors(vertices).Buffer(0.01)
	octree := NewOctree(aabb)

	for i := 0; i < reader.GetNumberOfVertices(); i++ {
		octree.Insert(vertices[i])
	}

	elapsed := time.Now().Sub(start).Milliseconds()
	fmt.Printf("Octree built (ms):  %d\n", elapsed)
	fmt.Printf("Number of items:    %d\n", octree.GetNumberOfItems())
	fmt.Printf("Number of nodes:    %d\n", octree.GetNumberOfNodes())
}
