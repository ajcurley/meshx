package halfedge

import (
	"fmt"
	"testing"
	"time"
)

func TestNewHalfEdgeMeshFromOBJPath(t *testing.T) {
	path := "/Users/acurley/projects/cfd/geometry/car.obj"

	start := time.Now()
	mesh, _ := NewHalfEdgeMeshFromOBJPath(path)
	elapsed := time.Now().Sub(start).Milliseconds()

	fmt.Printf("Loaded (ms):           %d\n", elapsed)
	fmt.Printf("Number of vertices:    %d\n", mesh.GetNumberOfVertices())
	fmt.Printf("Number of faces:       %d\n", mesh.GetNumberOfFaces())
	fmt.Printf("Number of half edges:  %d\n", mesh.GetNumberOfHalfEdges())
	fmt.Printf("Number of patches:     %d\n", mesh.GetNumberOfPatches())
	fmt.Printf("Number of components:  %d\n", len(mesh.GetComponents()))
	fmt.Printf("Is closed:             %v\n", mesh.IsClosed())
	fmt.Printf("Is consistent:         %v\n", mesh.IsConsistent())

	mesh.Orient()

	fmt.Printf("Is consistent (after): %v\n", mesh.IsConsistent())

	start = time.Now()
	mesh.WriteOBJPath("/Users/acurley/Desktop/mesh.obj")
	elapsed = time.Now().Sub(start).Milliseconds()

	fmt.Printf("Written (ms):          %d\n", elapsed)
}
