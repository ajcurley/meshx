use rustc_hash::{FxHashMap, FxHashSet};

use crate::geometry::{Aabb, Intersects};
use crate::spatial::{Search, SearchMany};

/// Maximum depth of an OctreeNode in an Octree
const MAX_DEPTH: usize = (std::mem::size_of::<usize>() * 8 - 1) / 3;

/// Maximum number of items that can be indexed on an OctreeNode
const MAX_ITEMS_PER_NODE: usize = 50;

#[derive(Debug, Clone)]
pub struct Octree<T>
where
    T: Intersects<Aabb>,
{
    nodes: FxHashMap<LocationalCode, OctreeNode>,
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

    /// Get a borrowed reference to an item
    pub fn item(&self, index: usize) -> &T {
        &self.items[index]
    }

    /// Get a borrowed reference to the items
    pub fn items(&self) -> &Vec<T> {
        &self.items
    }

    /// Get a borrowed reference to a node
    pub fn node(&self, code: LocationalCode) -> &OctreeNode {
        &self.nodes[&code]
    }

    /// Get a mutable reference to a node
    fn node_mut(&mut self, code: LocationalCode) -> &mut OctreeNode {
        self.nodes.get_mut(&code).expect("octree node not found")
    }

    /// Insert an item into the Octree. The item may be indexed on one or
    /// more nodes. Items must be strictly inside the Octree bounds.
    pub fn insert(&mut self, item: T) {
        let index = self.items.len();
        let mut queue = vec![LocationalCode::root()];
        let mut codes = vec![];

        while let Some(code) = queue.pop() {
            let node = self.node_mut(code);

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
    pub fn split(&mut self, code: LocationalCode) -> Vec<LocationalCode> {
        let node = self.node_mut(code);

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

impl<T, Q> Search<Q> for Octree<T>
where
    T: Intersects<Aabb> + Intersects<Q>,
    Q: Intersects<Aabb>,
{
    fn search(&self, query: &Q) -> Vec<usize> {
        let mut results = FxHashSet::default();
        let mut queue = vec![LocationalCode::root()];

        while let Some(code) = queue.pop() {
            let node = self.node(code);

            if query.intersects(&node.aabb) {
                if node.is_leaf {
                    for index in node.items.iter() {
                        if !results.contains(index) && self.items[*index].intersects(query) {
                            results.insert(*index);
                        }
                    }
                } else {
                    let mut children = node.children();
                    queue.append(&mut children);
                }
            }
        }

        results.into_iter().collect()
    }
}

impl<T, Q> SearchMany<Q> for Octree<T>
where
    T: Intersects<Aabb> + Intersects<Q> + Sync,
    Q: Intersects<Aabb> + Sync,
    Octree<T>: Search<Q>,
{
    fn search_many(&self, queries: &Vec<Q>) -> Vec<Vec<usize>> {
        let n_threads = std::thread::available_parallelism().unwrap().get();
        let n_queries = queries.len();
        let n = (n_queries as f64 / n_threads as f64).ceil() as usize;

        crossbeam::scope(|scope| {
            let mut futures = vec![];
            let mut results = vec![];

            for i in 0..n_threads {
                let j = (i * n).min(n_queries);
                let k = (j + n).min(n_queries);

                futures.push(scope.spawn(move |_| {
                    queries[j..k]
                        .iter()
                        .map(|q| self.search(q))
                        .collect::<Vec<Vec<usize>>>()
                }))
            }

            for future in futures {
                let mut result = future.join().unwrap();
                results.append(&mut result);
            }

            results
        })
        .unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct OctreeNode {
    code: LocationalCode,
    aabb: Aabb,
    is_leaf: bool,
    items: Vec<usize>,
}

impl OctreeNode {
    /// Construct an OctreeNode from its code and bounding box
    fn new(code: LocationalCode, aabb: Aabb) -> OctreeNode {
        OctreeNode {
            code,
            aabb,
            is_leaf: true,
            items: vec![],
        }
    }

    /// Construct a root OctreeNode from its bounding box
    fn new_root(aabb: Aabb) -> OctreeNode {
        OctreeNode::new(LocationalCode::root(), aabb)
    }

    /// Get the location code
    pub fn code(&self) -> LocationalCode {
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
        self.code.depth()
    }

    /// Get the children OctreeNode location codes
    pub fn children(&self) -> Vec<LocationalCode> {
        self.code.children()
    }

    /// Get if the node can be split
    pub fn can_split(&self) -> bool {
        self.is_leaf && self.depth() < MAX_DEPTH
    }

    /// Get if the node should be split
    fn should_split(&self) -> bool {
        self.can_split() && self.items.len() > MAX_ITEMS_PER_NODE
    }

    /// Get the codes for neighboring nodes of the same size
    fn face_neighbor(&self, direction: Direction) -> Option<usize> {
        /*
        let depth = self.depth();
        let mut octants = self.octants();
        let value = direction.value();
        let bits = octants.iter().map(|&o| direction.bit(o));

        // If all bits in the nodes path are on the same face as the
        // direction, then no neighbor exists.
        if bits.sum::<usize>() == depth * value {
            return None;
        }

        // Scanning from right to left, find the first bit that matches
        // the inverse of the direction's value. Reset the bit to the
        // direction's value and set all remaining octants to the inverse
        // of the direction's value.
        let mask = direction.mask();
        let shift = direction.shift();

        for octant in octants.iter_mut().rev() {
            if direction.bit(*octant) == 1 - value {
                *octant = (*octant & !mask) | (value << shift);
                break;
            }

            *octant = (*octant & !mask) | ((1 - value) << shift);
        }

        // Generate the code from the octants
        let code = code_from_octants(octants);
        Some(code)
        */
        unimplemented!()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct LocationalCode {
    code: usize,
}

impl LocationalCode {
    /// Construct a LocationalCode from its code value
    pub fn new(code: usize) -> LocationalCode {
        LocationalCode { code }
    }

    /// Construct a root LocationalCode
    pub fn root() -> LocationalCode {
        LocationalCode::new(1)
    }

    /// Construct a LocationalCode from its octant path
    pub fn from_path(path: &[usize]) -> LocationalCode {
        let mut code: usize = 0;

        for octant in path.iter() {
            code = code << 3 | octant;
        }

        LocationalCode::new(code)
    }

    /// Get the depth
    pub fn depth(&self) -> usize {
        for d in 0..MAX_DEPTH + 1 {
            if self.code >> 3 * d == 1 {
                return d;
            }
        }

        panic!("invalid location code");
    }

    /// Get the octant path
    pub fn path(&self) -> Vec<usize> {
        unimplemented!()
    }

    /// Get the children LocationalCodes
    pub fn children(&self) -> Vec<LocationalCode> {
        (0..8)
            .map(|o| LocationalCode::new(self.code << 3 | o))
            .collect()
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    East,
    West,
    North,
    South,
    Front,
    Back,
}

impl Direction {
    /// Get the bit shift for the significant bit
    pub fn shift(&self) -> usize {
        match self {
            Direction::East | Direction::West => 2,
            Direction::North | Direction::South => 1,
            Direction::Front | Direction::Back => 0,
        }
    }

    /// Get the bit mask for the significant bit
    pub fn mask(&self) -> usize {
        match self {
            Direction::East | Direction::West => 4,
            Direction::North | Direction::South => 2,
            Direction::Front | Direction::Back => 1,
        }
    }

    /// Get the bit value for the significant bit which represents the
    /// halfspace (0 or 1) of the octant
    pub fn value(&self) -> usize {
        match self {
            Direction::West | Direction::South | Direction::Front => 0,
            Direction::East | Direction::North | Direction::Back => 1,
        }
    }

    /// Get the value of the significant bit
    pub fn bit(&self, octant: usize) -> usize {
        (octant & self.mask()) >> self.shift()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::geometry::Vector3;

    #[test]
    fn test_insert() {
        let aabb = Aabb::unit();
        let mut octree = Octree::<Vector3>::new(aabb);

        let point = Vector3::zeros();
        octree.insert(point);

        assert_eq!(octree.nodes.len(), 1);
        assert_eq!(octree.items.len(), 1);

        let items = octree.node(1).items();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0], 0);
    }

    #[test]
    fn test_insert_split() {
        let aabb = Aabb::unit();
        let mut octree = Octree::<Vector3>::new(aabb);

        for i in 0..51 {
            let value = (i as f64) / 100. - 0.25;
            let point = Vector3::new(value, value, value);
            octree.insert(point);
        }

        assert_eq!(octree.nodes.len(), 9);
        assert_eq!(octree.items.len(), 51);

        assert_eq!(octree.node(8).items.len(), 26);
        assert_eq!(octree.node(9).items.len(), 1);
        assert_eq!(octree.node(10).items.len(), 1);
        assert_eq!(octree.node(11).items.len(), 1);
        assert_eq!(octree.node(12).items.len(), 1);
        assert_eq!(octree.node(13).items.len(), 1);
        assert_eq!(octree.node(14).items.len(), 1);
        assert_eq!(octree.node(15).items.len(), 26);
    }

    #[test]
    #[should_panic]
    fn test_insert_outside() {
        let aabb = Aabb::unit();
        let mut octree = Octree::<Vector3>::new(aabb);

        let point = Vector3::ones();
        octree.insert(point);
    }

    #[test]
    fn test_search() {
        let aabb = Aabb::unit();
        let mut octree = Octree::<Vector3>::new(aabb);

        for i in 0..51 {
            let value = (i as f64) / 100. - 0.25;
            let point = Vector3::new(value, value, value);
            octree.insert(point);
        }

        let center = Vector3::new(0.2, 0.2, 0.2);
        let halfsize = Vector3::new(0.05, 0.05, 0.05);
        let query = Aabb::new(center, halfsize);
        let results = octree.search(&query);

        assert_eq!(results.len(), 11);
    }

    #[test]
    fn test_search_no_results() {
        let aabb = Aabb::unit();
        let mut octree = Octree::<Vector3>::new(aabb);

        for i in 0..51 {
            let value = (i as f64) / 100. - 0.25;
            let point = Vector3::new(value, value, value);
            octree.insert(point);
        }

        let center = Vector3::new(0.2, -0.2, 0.2);
        let halfsize = Vector3::new(0.05, 0.05, 0.05);
        let query = Aabb::new(center, halfsize);
        let results = octree.search(&query);

        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_search_many() {
        let aabb = Aabb::unit();
        let mut octree = Octree::<Vector3>::new(aabb);

        for i in 0..51 {
            let value = (i as f64) / 100. - 0.25;
            let point = Vector3::new(value, value, value);
            octree.insert(point);
        }

        let center = Vector3::new(0.2, 0.2, 0.2);
        let halfsize = Vector3::new(0.05, 0.05, 0.05);
        let query1 = Aabb::new(center, halfsize);

        let center = Vector3::new(0.2, -0.2, 0.2);
        let halfsize = Vector3::new(0.05, 0.05, 0.05);
        let query2 = Aabb::new(center, halfsize);

        let queries = vec![query1, query2];
        let results = octree.search_many(&queries);

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].len(), 11);
        assert_eq!(results[1].len(), 0);
    }

    /*
    #[test]
    fn test_octree_node_octants() {
        let node = OctreeNode::new(474635, Aabb::unit());
        let octants = node.octants();

        assert_eq!(octants.len(), 6);
        assert_eq!(octants[0], 6);
        assert_eq!(octants[1], 3);
        assert_eq!(octants[2], 7);
        assert_eq!(octants[3], 0);
        assert_eq!(octants[4], 1);
        assert_eq!(octants[5], 3);
    }

    #[test]
    fn test_octree_node_face_neighbor_west() {
        let node = OctreeNode::new(474635, Aabb::unit());
        let neighbor = node.face_neighbor(Direction::West).unwrap();

        assert_eq!(neighbor, 472879);
    }

    #[test]
    fn test_octree_node_face_neighbor_north() {
        let code = code_from_octants(vec![6, 1, 5, 6, 2]);
        let expected = code_from_octants(vec![6, 1, 7, 4, 0]);

        let node = OctreeNode::new(code, Aabb::unit());
        let neighbor = node.face_neighbor(Direction::North).unwrap();

        assert_eq!(neighbor, expected);
    }
    */
}
