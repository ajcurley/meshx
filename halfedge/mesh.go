package halfedge

import (
	"github.com/ajcurley/meshx"
)

// Index-based half edge mesh data structure for manifold polygonal meshes.
type HalfEdgeMesh struct {
	vertices  []Vertex
	faces     []Face
	halfEdges []HalfEdge
	patches   []Patch
}

// Construct a HalfEdgeMesh from an OBJ file path.
func NewHalfEdgeMeshFromOBJPath(path string) (*HalfEdgeMesh, error) {
	panic("not implemented")
}

// Get the number of vertices.
func (m *HalfEdgeMesh) GetNumberOfVertices() int {
	return len(m.vertices)
}

// Get a vertex by index.
func (m *HalfEdgeMesh) GetVertex(index int) *Vertex {
	return &m.vertices[index]
}

// Get the faces using a vertex.
func (m *HalfEdgeMesh) GetVertexFaces(index int) []int {
	panic("not implemented")
}

// Get the outgoing half edges of a vertex.
func (m *HalfEdgeMesh) GetVertexOutgoingHalfEdges(index int) []int {
	panic("not implemented")
}

// Get the incoming half edges of a vertex.
func (m *HalfEdgeMesh) GetVertexIncomingHalfEdges(index int) []int {
	panic("not implemented")
}

// Get the number of faces.
func (m *HalfEdgeMesh) GetNumberOfFaces() int {
	return len(m.faces)
}

// Get a face by index.
func (m *HalfEdgeMesh) GetFace(index int) *Face {
	return &m.faces[index]
}

// Get the vertices of a face.
func (m *HalfEdgeMesh) GetFaceVertices(index int) []int {
	halfEdges := m.GetFaceHalfEdges(index)
	vertices := make([]int, len(halfEdges))

	for i, id := range halfEdges {
		vertices[i] = m.GetHalfEdge(id).Origin
	}

	return vertices
}

// Get the half edges of a face.
func (m *HalfEdgeMesh) GetFaceHalfEdges(index int) []int {
	face := m.GetFace(index)
	next := face.HalfEdge
	halfEdges := make([]int, 0, 3)

	for {
		halfEdges = append(halfEdges, next)
		next = m.GetHalfEdge(next).Next

		if next == face.HalfEdge {
			break
		}
	}

	return halfEdges
}

// Get the neighboring faces of a face.
func (m *HalfEdgeMesh) GetFaceNeighbors(index int) []int {
	halfEdges := m.GetFaceHalfEdges(index)
	faces := make([]int, 0, len(halfEdges))

	for _, id := range halfEdges {
		if halfEdge := m.GetHalfEdge(id); !halfEdge.IsBoundary() {
			twin := m.GetHalfEdge(halfEdge.Twin)
			faces = append(faces, twin.Face)
		}
	}

	return faces
}

// Get the number of half edges.
func (m *HalfEdgeMesh) GetNumberOfHalfEdges() int {
	return len(m.halfEdges)
}

// Get a half edge by index.
func (m *HalfEdgeMesh) GetHalfEdge(index int) *HalfEdge {
	return &m.halfEdges[index]
}

// Get the number of patches.
func (m *HalfEdgeMesh) GetNumberOfPatches() int {
	return len(m.patches)
}

// Get a patch by index.
func (m *HalfEdgeMesh) GetPatch(index int) *Patch {
	return &m.patches[index]
}

// Return true if there are no open edges.
func (m *HalfEdgeMesh) IsClosed() bool {
	for _, halfEdge := range m.halfEdges {
		if halfEdge.IsBoundary() {
			return false
		}
	}
	return true
}

// Get the isolated components (faces).
func (m *HalfEdgeMesh) GetComponents() [][]int {
	panic("not implemented")
}

// Return true if all neighboring faces share the same orientation.
func (m *HalfEdgeMesh) IsConsistent() bool {
	for _, halfEdge := range m.halfEdges {
		if !halfEdge.IsBoundary() {
			if m.GetHalfEdge(halfEdge.Twin).Origin == halfEdge.Origin {
				return false
			}
		}
	}
	return true
}

// Orient the mesh such that the faces of each component are consistent.
func (m *HalfEdgeMesh) Orient() error {
	panic("not implemented")
}

// Orient the mesh such that all the faces are consistently oriented relative
// to a reference point considered inside the domain.
func (m *HalfEdgeMesh) OrientWithReference(reference meshx.Vector) error {
	panic("not implemented")
}

// Merge two meshes together (in place).
func (m *HalfEdgeMesh) Merge(n *HalfEdgeMesh) {
	panic("not implemented")
}

// Extract the faces into a new mesh.
func (m *HalfEdgeMesh) Extract(faces []int) *HalfEdgeMesh {
	panic("not implemented")
}
