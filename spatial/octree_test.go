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
	//path := "/Users/acurley/projects/cfd/geometry/radiator_oil_core.obj"
	reader, err := exchange.ReadOBJFromPath(path)
	assert.Empty(t, err)

	vertices := make([]meshx.Vector, reader.GetNumberOfVertices())
	triangles := make([]meshx.Triangle, reader.GetNumberOfFaces())

	for i := 0; i < reader.GetNumberOfVertices(); i++ {
		vertices[i] = reader.GetVertex(i)
	}

	for i := 0; i < reader.GetNumberOfFaces(); i++ {
		face := reader.GetFace(i)
		p := vertices[face[0]]
		q := vertices[face[1]]
		r := vertices[face[2]]
		triangles[i] = meshx.NewTriangle(p, q, r)
	}

	start := time.Now()

	aabb := meshx.NewAABBFromVectors(vertices).Buffer(0.01)
	octree := NewOctree(aabb)

	for i := 0; i < len(triangles); i++ {
		if err := octree.Insert(triangles[i]); err != nil {
			t.Fatal(err)
		}
	}

	elapsed := time.Now().Sub(start).Milliseconds()
	fmt.Println("")
	fmt.Printf("Octree built (ms):  %d\n", elapsed)
	fmt.Printf("Number of items:    %d\n", octree.GetNumberOfItems())
	fmt.Printf("Number of nodes:    %d\n", octree.GetNumberOfNodes())

	start = time.Now()
	ray := meshx.NewRay(meshx.Vector{0, 0, -1}, meshx.Vector{0, 0, 1})
	results := octree.Query(ray)
	elapsed = time.Now().Sub(start).Microseconds()

	fmt.Println("")
	fmt.Printf("Octree query (us):  %d\n", elapsed)
	fmt.Printf("Number of items:    %d\n", len(results))
}
