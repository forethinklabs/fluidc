//! Fluid Communities algorithm for community detection.

#![warn(missing_docs)]

use petgraph::{graph::NodeIndex, Graph};

/// Fluid Communities - A highly scalable community detection algorithm.
pub fn fluidc<N, E, Ty, Ix>(
    graph: &Graph<N, E, Ty, Ix>,
    communities: u8,
    max_iter: Option<u8>,
) -> impl Iterator<Item = (u8, NodeIndex)> {
    std::iter::empty()
}
