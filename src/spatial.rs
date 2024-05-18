pub mod octree;

// Re-exports
pub use octree::Octree;

/// Search for the unique set of indexed items spatially intersecting
/// the query geometry.
pub trait Search<Q> {
    fn search(&self, query: &Q) -> Vec<usize>;
}

/// Search for the unique set of indexed items spatially intersecting
/// each of the query geometries. This uses the maximum available threads.
pub trait SearchMany<Q> {
    fn search_many(&self, queries: &Vec<Q>) -> Vec<Vec<usize>>;
}
