use std::collections::HashSet;
use crate::solver::graph::grid_n_d::GridND;

pub mod grid_n_d;
pub mod erdos_renyi;

/// Graph trait. Implements number of points, and getting neighbors of a particular point.
///
/// Vertices are u64 so not every vertex has to be explicitly specified by the object. The number of
///  vertices has to be small: we loop over every vertex once in the solver and sample from a distribution
///  with every vertex every time a state change. Very doable is 40*40 = 1600 points, slow but
///  doable is 240*240 = 57600 points.
///
/// Directed, does not allow multi-edges, does allow self-loops (by the format of the get_neighbors function).
/// It's not entirely clear what a self-loop means in the context of an interacting particle system.
///
/// For most applications, the edges will be undirected, and there will be no self-loops.
pub trait Graph {
    /// Return the number of point (aka vertices, nodes) in the graph
    fn nr_points(&self) -> u64;

    /// Return a hash set of all the neighbors of a particular input point.
    fn get_neighbors(&self, _: u64) -> HashSet<u64>;
}

pub enum GraphKind {
    ErdosRenyi,
    GridND
}

pub fn graph_constructor(graph_kind: GraphKind, graph_parameters: Vec<u64>) -> Box<dyn Graph> {
    match graph_kind {
        GraphKind::GridND => {
            Box::new(
                GridND::from(graph_parameters)
            )
        }
        _ => {
            todo!()
        }
    }
}