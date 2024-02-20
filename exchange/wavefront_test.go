package exchange

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestReadOBJFromPath(t *testing.T) {
	path := "/Users/acurley/projects/cfd/geometry/car.obj"
	reader, err := ReadOBJFromPath(path)

	assert.Empty(t, err)
	assert.Equal(t, 2061978, reader.GetNumberOfVertices())
	assert.Equal(t, 4125772, reader.GetNumberOfFaces())
	assert.Equal(t, 181, reader.GetNumberOfPatches())
	assert.Equal(t, []int{1960351, 1958638, 1960352}, reader.GetFace(4125770))
	assert.Equal(t, []int{1960351, 1960352, 1958435}, reader.GetFace(4125771))
}
