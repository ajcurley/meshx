package meshx

import (
	"bytes"
	"compress/gzip"
	"testing"

	"github.com/stretchr/testify/assert"
)

// Read an OBJ file from path.
func TestReadOBJFromPath(t *testing.T) {
	path := "testdata/box.obj"
	mesh, err := ReadOBJFromPath(path)

	assert.Empty(t, err)
	assert.Equal(t, mesh.GetNumberOfVertices(), 24)
	assert.Equal(t, mesh.GetNumberOfFaces(), 12)
	assert.Equal(t, mesh.GetNumberOfPatches(), 0)
}

// Read an OBJ file from path (gzip).
func TestReadOBJFromPathGZIP(t *testing.T) {
	path := "testdata/box.obj.gz"
	mesh, err := ReadOBJFromPath(path)

	assert.Empty(t, err)
	assert.Equal(t, mesh.GetNumberOfVertices(), 24)
	assert.Equal(t, mesh.GetNumberOfFaces(), 12)
	assert.Equal(t, mesh.GetNumberOfPatches(), 0)
}

// Read an OBJ file from path with mixed elements and patches.
func TestReadOBJFromPathPatches(t *testing.T) {
	path := "testdata/box.patches.obj"
	mesh, err := ReadOBJFromPath(path)

	assert.Empty(t, err)
	assert.Equal(t, mesh.GetNumberOfVertices(), 8)
	assert.Equal(t, mesh.GetNumberOfFaces(), 7)
	assert.Equal(t, mesh.GetNumberOfPatches(), 6)
}

// Write an OBJ file.
func TestWriteOBJ(t *testing.T) {
	vertices := []Vector{
		NewVector(0, 0, 0),
		NewVector(0, 1, 0),
		NewVector(1, 1, 0),
	}

	faces := [][]int{
		[]int{0, 1, 2},
	}

	var expected string
	expected += "v 0.000000 0.000000 0.000000\n"
	expected += "v 0.000000 1.000000 0.000000\n"
	expected += "v 1.000000 1.000000 0.000000\n"
	expected += "f 1 2 3\n"

	var writer bytes.Buffer
	objWriter := NewOBJWriter(&writer)
	objWriter.SetVertices(vertices)
	objWriter.SetFaces(faces)

	err := objWriter.Write()
	assert.Empty(t, err)
	assert.Equal(t, expected, writer.String())
}

// Write an OBJ file (gzip).
func TestWriteOBJGZIP(t *testing.T) {
	vertices := []Vector{
		NewVector(0, 0, 0),
		NewVector(0, 1, 0),
		NewVector(1, 1, 0),
	}

	faces := [][]int{
		[]int{0, 1, 2},
	}

	var expected string
	expected += "v 0.000000 0.000000 0.000000\n"
	expected += "v 0.000000 1.000000 0.000000\n"
	expected += "v 1.000000 1.000000 0.000000\n"
	expected += "f 1 2 3\n"

	var expectedBuf bytes.Buffer
	expectedWriter := gzip.NewWriter(&expectedBuf)
	expectedWriter.Write([]byte(expected))

	var writer bytes.Buffer
	gzipWriter := gzip.NewWriter(&writer)
	objWriter := NewOBJWriter(gzipWriter)
	objWriter.SetVertices(vertices)
	objWriter.SetFaces(faces)

	err := objWriter.Write()
	assert.Empty(t, err)
	assert.Equal(t, expectedBuf.String(), writer.String())
}
