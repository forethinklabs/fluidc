//! Fluid Communities algorithm for community detection.

#![warn(missing_docs)]

use petgraph::graph::{IndexType, NodeIndex};
use petgraph::{EdgeType, Graph};
use rand::prelude::SliceRandom;
use std::collections::HashMap;

const MAX_DENSITY: f32 = 1.0;

const THRESHOLD: f32 = 0.0001;

const DEFAULT_ITER: u8 = 100;

/// Fluid Communities - A highly scalable community detection algorithm.
pub fn fluidc<N, E, Ty, Ix>(
    graph: &Graph<N, E, Ty, Ix>,
    max_communities: usize,
    max_iter: Option<u8>,
) -> HashMap<usize, Vec<NodeIndex<Ix>>>
where
    Ty: EdgeType,
    Ix: IndexType,
{
    // --- Establish initial randomness --- //
    let mut rng = rand::thread_rng();
    let mut vertices: Vec<_> = graph.node_indices().collect();
    vertices.shuffle(&mut rng);

    // --- Establish initial communities --- //
    let mut density = HashMap::new();
    let mut com_to_numvertices = HashMap::new();
    let communities: HashMap<NodeIndex<Ix>, usize> = vertices[0..max_communities]
        .into_iter()
        .enumerate()
        .map(|(i, n)| (*n, i))
        .collect();
    for i in communities.values() {
        com_to_numvertices.insert(i, 1);
        density.insert(i, MAX_DENSITY);
    }

    // --- Produce progressively more accurate communities --- //
    for i in 0..(max_iter.unwrap_or(DEFAULT_ITER)) {
        vertices.shuffle(&mut rng);
    }

    // --- Invert accumulated results --- //
    let mut res: HashMap<usize, Vec<_>> = HashMap::new();
    for (ix, com) in communities.into_iter() {
        let entry = res.entry(com).or_default();
        entry.push(ix);
    }
    res
}
