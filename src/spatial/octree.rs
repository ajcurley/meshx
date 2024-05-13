use rustc_hash::FxHashMap;

use crate::geometry::{Aabb, Intersects};

/// Maximum depth of an OctreeNode in an Octree
const MAX_DEPTH: usize = (std::mem::size_of::<usize>() * 8 - 1) / 3;

/// Maximum number of items that can be indexed on an OctreeNode
const MAX_ITEMS_PER_NODE: usize = 50;

#[derive(Debug, Clone)]
pub struct Octree<T>
where
    T: Intersects<Aabb>,
{
    nodes: FxHashMap<usize, OctreeNode>,
    items: Vec<T>,
}

impl<T> Octree<T>
where
    T: Intersects<Aabb>,
{
    /// Construct an Octree from its bounding box
    pub fn new(aabb: Aabb) -> Octree<T> {
        let node = OctreeNode::new_root(aabb);
        let mut nodes = FxHashMap::default();
        nodes.insert(node.code, node);

        Octree {
            nodes,
            items: vec![],
        }
    }

    /// Get a borrowed reference to the items
    pub fn items(&self) -> &Vec<T> {
        &self.items
    }

    /// Insert an item
    pub fn insert(&mut self, item: T) {
        let index = self.items.len();
        let mut queue = vec![1];
        let mut codes = vec![];

        while let Some(code) = queue.pop() {
            let node = self.nodes.get_mut(&code).expect("octree node not found");

            if item.intersects(&node.aabb) {
                if node.is_leaf {
                    node.items.push(index);
                    codes.push(code);
                } else {
                    let mut children = node.children();
                    queue.append(&mut children);
                }
            }
        }

        if codes.is_empty() {
            panic!("item not inserted");
        }

        self.items.push(item);

        for code in codes {
            if self.nodes[&code].should_split() {
                self.split(code);
            }
        }
    }

    /// Split an internal (non-leaf) node and redistribute any indexed
    /// items amongst the children leaf nodes.
    pub fn split(&mut self, code: usize) -> Vec<usize> {
        let node = self.nodes.get_mut(&code).expect("octree node not found");

        if !node.can_split() {
            panic!("octree node cannot be split");
        }

        let children = node.children();
        let items = node.items.clone();
        let aabb = node.aabb();

        node.is_leaf = false;
        node.items.clear();

        for (octant, &child_code) in children.iter().enumerate() {
            let child_aabb = aabb.octant(octant);
            let mut child_node = OctreeNode::new(child_code, child_aabb);

            for &index in &items {
                if self.items[index].intersects(&child_aabb) {
                    child_node.items.push(index);
                }
            }

            self.nodes.insert(child_code, child_node);
        }

        children
    }
}

#[derive(Debug, Clone)]
pub struct OctreeNode {
    code: usize,
    aabb: Aabb,
    is_leaf: bool,
    items: Vec<usize>,
}

impl OctreeNode {
    /// Construct an OctreeNode from its code and bounding box
    fn new(code: usize, aabb: Aabb) -> OctreeNode {
        OctreeNode {
            code,
            aabb,
            is_leaf: true,
            items: vec![],
        }
    }

    /// Construct a root OctreeNode from its bounding box
    fn new_root(aabb: Aabb) -> OctreeNode {
        OctreeNode::new(1, aabb)
    }

    /// Get the location code
    pub fn code(&self) -> usize {
        self.code
    }

    /// Get the axis-aligned bounding box
    pub fn aabb(&self) -> Aabb {
        self.aabb
    }

    /// Get if the node is a leaf
    pub fn is_leaf(&self) -> bool {
        self.is_leaf
    }

    /// Get a borrowed reference to the items
    pub fn items(&self) -> &Vec<usize> {
        &self.items
    }

    /// Compute the depth of the code
    pub fn depth(&self) -> usize {
        for d in 0..MAX_DEPTH + 1 {
            if self.code >> 3 * d == 1 {
                return d;
            }
        }

        panic!("invalid location code");
    }

    /// Get the children OctreeNode location codes
    pub fn children(&self) -> Vec<usize> {
        let mut codes = vec![0; 8];

        for (octant, code) in codes.iter_mut().enumerate() {
            *code = (self.code << 3) | octant;
        }

        codes
    }

    /// Get if the node can be split
    pub fn can_split(&self) -> bool {
        self.is_leaf && self.depth() < MAX_DEPTH
    }

    /// Get if the node should be split
    fn should_split(&self) -> bool {
        self.can_split() && self.items.len() > MAX_ITEMS_PER_NODE
    }
}
