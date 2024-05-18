pub mod octree;

// Re-exports
pub use octree::Octree;

/// Search for the unique set of indexed items spatially intersecting
/// the query geometry.
pub trait Search<Q> {
    fn search(&self, query: &Q) -> Vec<usize>;
}
