//! Fluid Communities algorithm for community detection.

#![warn(missing_docs)]

use petgraph::graph::{IndexType, NodeIndex};
use petgraph::{EdgeType, Graph};
use rand::prelude::SliceRandom;
use std::collections::HashMap;

const MAX_DENSITY: f32 = 1.0;

const THRESHOLD: f32 = 0.0001;

const DEFAULT_ITER: u8 = 100;

type Community = usize;

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
    let mut vertices: Vec<NodeIndex<Ix>> = graph.node_indices().collect();
    vertices.shuffle(&mut rng);

    // --- Establish initial communities --- //
    let mut density: HashMap<Community, f32> = HashMap::new();
    let mut com_to_numvertices: HashMap<Community, usize> = HashMap::new();
    let mut communities: HashMap<NodeIndex<Ix>, Community> = vertices[0..max_communities]
        .into_iter()
        .enumerate()
        .map(|(i, n)| (*n, i))
        .collect();
    for i in communities.values() {
        com_to_numvertices.insert(*i, 1);
        density.insert(*i, MAX_DENSITY);
    }

    // --- Produce progressively more accurate communities --- //
    for _ in 0..(max_iter.unwrap_or(DEFAULT_ITER)) {
        vertices.shuffle(&mut rng);

        for vertex in vertices.iter() {
            let mut com_counter: HashMap<Community, f32> = HashMap::new();

            // --- Take into account self vertex community --- //
            if let Some(com) = communities.get(vertex) {
                if let Some(den) = density.get(com) {
                    if com_counter.contains_key(com) {
                        com_counter.entry(*com).and_modify(|d| *d += den);
                    }
                }
            }

            // --- Gather neighbour vertex communities --- //
            // TODO Do we want directed or undirected neighbours?
            for v in graph.neighbors_undirected(*vertex) {
                if let Some(com) = communities.get(&v) {
                    if let Some(den) = density.get(com) {
                        if com_counter.contains_key(com) {
                            com_counter.entry(*com).and_modify(|d| *d += den);
                        }
                    }
                }
            }

            // --- Check which is the community with highest density --- //
            if let Some(max_freq) = com_counter.values().copied().reduce(f32::max) {
                let best_communities: Vec<_> = com_counter
                    .into_iter()
                    .filter(|(_, freq)| (max_freq - freq) < THRESHOLD)
                    .map(|(com, _)| com)
                    .collect();

                // --- If actual vertex com in best communities, it is preserved --- //
                if communities
                    .get(vertex)
                    .and_then(|com| best_communities.contains(com).then(|| ()))
                    .is_none()
                {
                    // TODO Handle halting via `cont`.
                    // --- If vertex community changes... --- //
                    // FIXME Panic risk! How do we know this isn't empty?
                    let new_com = best_communities.choose(&mut rng).unwrap();

                    // --- Update previous community status --- //
                    if let Some(com) = communities.get(vertex) {
                        // TODO Check if this causes underflows.
                        // Although, doesn't Rust panic when that happens?
                        com_to_numvertices
                            .entry(*com)
                            .and_modify(|count| *count -= 1);
                        if let Some(count) = com_to_numvertices.get(com) {
                            density.insert(*com, MAX_DENSITY / *count as f32);
                        }
                    }

                    // --- Update new community status --- //
                    communities.insert(*vertex, *new_com);
                    com_to_numvertices.entry(*new_com).and_modify(|n| *n += 1);
                    // FIXME Panic risk on the indexing?
                    density.insert(*new_com, MAX_DENSITY / com_to_numvertices[new_com] as f32);
                }
            }
        }
    }

    // --- Invert accumulated results --- //
    let mut res: HashMap<usize, Vec<_>> = HashMap::new();
    for (ix, com) in communities.into_iter() {
        let entry = res.entry(com).or_default();
        entry.push(ix);
    }
    res
}
