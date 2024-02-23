package halfedge

import (
	"compress/gzip"
	"io"
	"os"
	"path/filepath"
	"strings"

	"github.com/ajcurley/meshx"
)

// Index-based half edge mesh data structure for manifold polygonal meshes.
type HalfEdgeMesh struct {
	vertices  []Vertex
	faces     []Face
	halfEdges []HalfEdge
	patches   []Patch
}

// Construct a HalfEdgeMesh from a MeshReader.
func NewHalfEdgeMesh(source meshx.Mesh) (*HalfEdgeMesh, error) {
	mesh := HalfEdgeMesh{
		vertices:  make([]Vertex, source.GetNumberOfVertices()),
		faces:     make([]Face, source.GetNumberOfFaces()),
		halfEdges: make([]HalfEdge, source.GetNumberOfFaceEdges()),
		patches:   make([]Patch, source.GetNumberOfPatches()),
	}

	for i := range source.GetNumberOfPatches() {
		mesh.patches[i] = Patch{source.GetPatch(i)}
	}

	for i := range source.GetNumberOfVertices() {
		mesh.vertices[i] = Vertex{source.GetVertex(i), -1}
	}

	var nHalfEdges int
	sharedEdges := make(map[[2]int]int)

	for i := range source.GetNumberOfFaces() {
		face := source.GetFace(i)
		facePatch := source.GetFacePatch(i)
		mesh.faces[i] = Face{nHalfEdges, facePatch}

		for j, vertex := range face {
			k := nHalfEdges + j
			next := (j + 1) % len(face)
			prev := (j - 1) % len(face)
			prev -= len(face) * min(0, prev)

			mesh.halfEdges[k] = HalfEdge{
				Origin: vertex,
				Face:   i,
				Next:   nHalfEdges + next,
				Prev:   nHalfEdges + prev,
				Twin:   -1,
			}

			p := min(vertex, face[next])
			q := max(vertex, face[next])
			edge := [2]int{p, q}

			if twin, ok := sharedEdges[edge]; ok {
				mesh.halfEdges[k].Twin = twin
				mesh.halfEdges[twin].Twin = k
				delete(sharedEdges, edge)
			} else {
				sharedEdges[edge] = k
			}
		}

		nHalfEdges += len(face)
	}

	if len(sharedEdges) != 0 {
		return nil, meshx.ErrNonManifold
	}

	return &mesh, nil
}

// Construct a HalfEdgeMesh from an OBJ file reader.
func NewHalfEdgeMeshFromOBJ(reader io.Reader) (*HalfEdgeMesh, error) {
	source := meshx.NewOBJReader(reader)

	if err := source.Read(); err != nil {
		return nil, err
	}

	return NewHalfEdgeMesh(source)
}

// Construct a HalfEdgeMesh from an OBJ file path.
func NewHalfEdgeMeshFromOBJPath(path string) (*HalfEdgeMesh, error) {
	source, err := meshx.ReadOBJFromPath(path)
	if err != nil {
		return nil, err
	}
	return NewHalfEdgeMesh(source)
}

// Write the HalfEdgeMesh to an OBJ file.
func (m *HalfEdgeMesh) WriteOBJ(writer io.Writer) error {
	vertices := make([]meshx.Vector, m.GetNumberOfVertices())
	faces := make([][]int, m.GetNumberOfFaces())
	facePatches := make([]int, m.GetNumberOfFaces())
	patches := make([]string, m.GetNumberOfPatches())

	for i := range m.GetNumberOfPatches() {
		patches[i] = m.patches[i].Name
	}

	for i := range m.GetNumberOfVertices() {
		vertices[i] = m.vertices[i].Point
	}

	for i := range m.GetNumberOfFaces() {
		faces[i] = m.GetFaceVertices(i)
		facePatches[i] = m.faces[i].Patch
	}

	objWriter := meshx.NewOBJWriter(writer)
	objWriter.SetVertices(vertices)
	objWriter.SetFaces(faces)
	objWriter.SetFacePatches(facePatches)
	objWriter.SetPatches(patches)

	return objWriter.Write()
}

// Write the HalfEdgeMesh to an OBJ file path.
func (m *HalfEdgeMesh) WriteOBJPath(path string) error {
	file, err := os.Create(path)
	if err != nil {
		return err
	}
	defer file.Close()

	var writer io.Writer

	if strings.ToLower(filepath.Ext(path)) == ".gz" {
		gzipFile := gzip.NewWriter(file)
		defer gzipFile.Close()
		writer = gzipFile
	} else {
		writer = file
	}

	return m.WriteOBJ(writer)
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

// Flip the orientation of a face.
func (m *HalfEdgeMesh) flipFace(index int) {
	for _, id := range m.GetFaceHalfEdges(index) {
		halfEdge := m.GetHalfEdge(id)
		origin := m.GetHalfEdge(halfEdge.Next).Origin

		m.halfEdges[id] = HalfEdge{
			Origin: origin,
			Face:   halfEdge.Face,
			Next:   halfEdge.Prev,
			Prev:   halfEdge.Next,
			Twin:   halfEdge.Twin,
		}
	}
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
	components := make([][]int, 0)
	visited := make([]bool, m.GetNumberOfFaces())

	for i := 0; i < m.GetNumberOfFaces(); i++ {
		if !visited[i] {
			var current int
			component := make([]int, 0)
			queue := []int{i}

			for len(queue) > 0 {
				current, queue = queue[0], queue[1:]

				if !visited[current] {
					visited[current] = true
					component = append(component, current)

					for _, neighbor := range m.GetFaceNeighbors(current) {
						if !visited[neighbor] {
							queue = append(queue, neighbor)
						}
					}

				}
			}

			components = append(components, component)
		}
	}

	return components
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

// Return true if all neighboring faces share the same orientation for
// each component relative to the reference.
func (m *HalfEdgeMesh) IsConsistentWithReference(reference meshx.Vector) bool {
	panic("not implemented")
}

// Orient the mesh such that the faces of each component are consistent.
func (m *HalfEdgeMesh) Orient() {
	if m.IsConsistent() {
		return
	}

	visited := make([]bool, m.GetNumberOfFaces())

	for i := 0; i < m.GetNumberOfFaces(); i++ {
		if !visited[i] {
			var current int
			queue := []int{i}

			for n := len(queue); n > 0; n = len(queue) {
				current, queue = queue[n-1], queue[:n-1]

				if !visited[current] {
					visited[current] = true

					for _, neighbor := range m.GetFaceNeighbors(current) {
						if !m.checkFaceOrientation(current, neighbor) {
							visited[current] = true
							m.flipFace(neighbor)
						} else {
							queue = append(queue, neighbor)
						}
					}
				}
			}
		}
	}
}

// Orient the mesh such that all the faces are consistently oriented relative
// to a reference point considered inside the domain.
func (m *HalfEdgeMesh) OrientWithReference(reference meshx.Vector) error {
	panic("not implemented")
}

// Check two adjacent faces for consistent orientation.
func (m *HalfEdgeMesh) checkFaceOrientation(source, target int) bool {
	for _, id := range m.GetFaceHalfEdges(source) {
		halfEdge := m.GetHalfEdge(id)

		if !halfEdge.IsBoundary() {
			if twin := m.GetHalfEdge(halfEdge.Twin); twin.Face == target {
				return halfEdge.Origin != twin.Origin
			}
		}
	}
	return false
}

// Merge two meshes together (in place).
func (m *HalfEdgeMesh) Merge(n *HalfEdgeMesh) {
	offsetVertex := m.GetNumberOfVertices()
	offsetFace := m.GetNumberOfFaces()
	offsetHalfEdge := m.GetNumberOfHalfEdges()
	offsetPatch := m.GetNumberOfPatches()

	for _, vertex := range n.vertices {
		m.vertices = append(m.vertices, vertex)
	}

	for _, face := range n.faces {
		face.HalfEdge += offsetHalfEdge
		face.Patch += offsetPatch
		m.faces = append(m.faces, face)
	}

	for _, halfEdge := range n.halfEdges {
		halfEdge.Origin += offsetVertex
		halfEdge.Face += offsetFace
		halfEdge.Next += offsetHalfEdge
		halfEdge.Prev += offsetHalfEdge

		if !halfEdge.IsBoundary() {
			halfEdge.Twin += offsetHalfEdge
		}

		m.halfEdges = append(m.halfEdges, halfEdge)
	}

	for _, patch := range n.patches {
		m.patches = append(m.patches, patch)
	}
}

// Extract the faces into a new mesh.
func (m *HalfEdgeMesh) Extract(faces []int) *HalfEdgeMesh {
	panic("not implemented")
}

// Translate the mesh by a Vector.
func (m *HalfEdgeMesh) Translate(offset meshx.Vector) {
	for i, vertex := range m.vertices {
		m.vertices[i] = Vertex{
			Point:    vertex.Point.Add(offset),
			HalfEdge: vertex.HalfEdge,
		}
	}
}
