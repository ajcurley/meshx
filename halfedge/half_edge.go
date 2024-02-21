package halfedge

type HalfEdge struct {
	Origin int
	Face   int
	Next   int
	Prev   int
	Twin   int
}

// Return true if the half edge is on the boundary (no twin).
func (h HalfEdge) IsBoundary() bool {
	return h.Twin < 0
}
