package spatial

import (
	"errors"

	"github.com/ajcurley/meshx"
)

const (
	OctreeMaxDepth     = 21
	OctreeMaxLeafItems = 100
)

var (
	ErrOctreeItemNotInserted = errors.New("item not inserted")
	ErrOctreeCannotSplitNode = errors.New("cannot split node")
)

type Octree struct {
	nodes map[uint64]*OctreeNode
	items []meshx.IntersectsAABB
}

// Construct a bounded octree.
func NewOctree(aabb meshx.AABB) *Octree {
	return &Octree{
		nodes: map[uint64]*OctreeNode{1: NewOctreeNode(1, aabb)},
		items: make([]meshx.IntersectsAABB, 0),
	}
}

// Insert an item into the octree.
func (o *Octree) Insert(item meshx.IntersectsAABB) error {
	var code uint64

	codes := []uint64{}
	queue := []uint64{1}
	index := len(o.items)

	for len(queue) > 0 {
		code, queue = queue[0], queue[1:]
		node := o.nodes[code]

		if item.IntersectsAABB(node.aabb) {
			if node.isLeaf {
				codes = append(codes, code)
			} else {
				children := node.Children()
				queue = append(queue, children...)
			}
		}
	}

	if len(codes) == 0 {
		return ErrOctreeItemNotInserted
	}

	o.items = append(o.items, item)

	for _, code := range codes {
		node := o.nodes[code]
		node.items = append(node.items, index)

		if node.shouldSplit() {
			o.Split(code)
		}
	}

	return nil
}

// Split a leaf octree node into its eight octant children.
func (o *Octree) Split(code uint64) error {
	node := o.nodes[code]

	if !node.canSplit() {
		return ErrOctreeCannotSplitNode
	}

	for octant, childCode := range node.Children() {
		aabb := node.aabb.Octant(octant)
		childNode := NewOctreeNode(childCode, aabb)

		for _, index := range node.items {
			if o.items[index].IntersectsAABB(aabb) {
				childNode.items = append(childNode.items, index)
			}
		}

		o.nodes[childCode] = childNode
	}

	clear(node.items)
	node.isLeaf = false

	return nil
}

// Query the octree for intersection items.
func (o *Octree) Query(query meshx.IntersectsAABB) []int {
	panic("not implemented")
}

type OctreeNode struct {
	items  []int
	aabb   meshx.AABB
	code   uint64
	isLeaf bool
}

// Construct a leaf OctreeNode.
func NewOctreeNode(code uint64, aabb meshx.AABB) *OctreeNode {
	return &OctreeNode{
		items:  make([]int, 0),
		aabb:   aabb,
		code:   code,
		isLeaf: true,
	}
}

// Compute the depth from the code.
func (o *OctreeNode) Depth() int {
	for depth := 0; depth <= OctreeMaxDepth; depth++ {
		if o.code>>(3*depth) == 1 {
			return depth
		}
	}

	panic("invalid octree code")
}

// Compute the children octant codes.
func (o *OctreeNode) Children() []uint64 {
	children := make([]uint64, 8)

	for octant := range children {
		children[octant] = o.code<<3 | uint64(octant)
	}

	return children
}

// Return true if the node can be split.
func (o *OctreeNode) canSplit() bool {
	return o.isLeaf && o.Depth() < OctreeMaxDepth
}

// Return true if the node should be split.
func (o *OctreeNode) shouldSplit() bool {
	return o.canSplit() && len(o.items) > OctreeMaxLeafItems
}
