use std::collections::HashSet;

pub mod grid_n_d;
pub mod erdos_renyi;
pub mod diluted_lattice;

/// Graph trait. Implements number of points, and getting neighbors of a particular point.
///
/// Vertices are usize so not every vertex has to be explicitly specified by the object. The number of
///  vertices has to be small: we loop over every vertex once in the solver and sample from a distribution
///  with every vertex every time a state change. Very doable is 40×40 = 1600 points, slow but
///  doable is 240×240 = 57600 points.
///
/// Directed, does not allow multi-edges, does allow self-loops (by the format of the get_neighbors function).
/// It's not entirely clear what a self-loop means in the context of an interacting particle system.
///
/// For IPS applications, the edges will be undirected, and there will be no self-loops.
///
/// Overwrite all methods for a graph implementation.
pub trait Graph {
    /// Return the number of point (aka vertices, nodes) in the graph. A list of all points is
    /// then `0..graph.nr_points()`.
    fn nr_points(&self) -> usize;

    /// Return a hash set of all the neighbors of a particular input point.
    fn get_neighbors(&self, particle: usize) -> HashSet<usize>;
    
    /// Print a description of the graph.
    fn describe(&self);
}