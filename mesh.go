package meshx

// Generic mesh reader interface.
type MeshReader interface {
	Read() error
	GetNumberOfVertices() int
	GetNumberOfFaces() int
	GetNumberOfFaceEdges() int
	GetNumberOfPatches() int
	GetVertex(int) Vector
	GetFace(int) []int
	GetFacePatch(int) int
	GetPatch(int) string
}

// Generic mesh writer interface.
type MeshWriter interface {
	Write() error
	SetVertices([]Vector)
	SetFaces([][]int)
	SetFacePatches([]int)
	SetPatches([]string)
}
