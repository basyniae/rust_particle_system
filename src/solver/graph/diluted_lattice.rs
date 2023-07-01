use std::collections::HashSet;
use rand::distributions::{Bernoulli, Distribution};
use rand::rngs::ThreadRng;
use crate::Graph;

/// i is connected to j with probability if i,j are adjacent in the corresponding lattice
pub struct DilutedLattice {
    nr_points: usize,
    dim_x: usize,
    step_x: usize,
    dim_y: usize,
    step_y: usize,
    probability: f64,
    is_edge: Vec<bool>, // A mask over the actual lattice's edges
}

impl Graph for DilutedLattice {
    fn nr_points(&self) -> usize {
        self.nr_points
    }

    fn get_neighbors(&self, particle: usize) -> HashSet<usize> {
        let mut running_neighbors = HashSet::new();
        // There are 4 potential neighbors from the associated lattice, which we compute as in
        // GridND. Then we consult the is_edge list to see if they are actual neighbors in diluted
        // lattice.

        let x_coord = particle % self.dim_x;
        let y_coord = particle / self.dim_x;

        // the right neighbor:
        if x_coord == self.dim_x - 1 { //  if on the far boundary
            if *self.is_edge.get(particle).unwrap() {
                running_neighbors.insert(particle + self.step_x - self.dim_x);
            }
        } else {
            if *self.is_edge.get(particle).unwrap() {
                running_neighbors.insert(particle + self.step_x);
            }
        }
        // the left neighbor:
        if x_coord == 0 { // if on the close boundary
            if *self.is_edge.get(particle + self.dim_x - self.step_x).unwrap() {
                running_neighbors.insert(particle + self.dim_x - self.step_x);
            }
        } else {
            if *self.is_edge.get(particle - self.step_x).unwrap() {
                running_neighbors.insert(particle - self.step_x);
            }
        }

        // the bottom neighbor:
        if y_coord == self.dim_y - 1 { // if on the far boundary
            if *self.is_edge.get(self.nr_points + particle).unwrap() {
                running_neighbors.insert(particle + self.step_y - self.nr_points); //WRONG?
            }
        } else {
            if *self.is_edge.get(self.nr_points + particle).unwrap() {
                running_neighbors.insert(particle + self.step_y);
            }
        }
        // the top neighbor:
        if y_coord == 0 { // if on the close boundary
            if *self.is_edge.get(2 * self.nr_points + particle - self.step_y).unwrap() {
                running_neighbors.insert(particle + self.nr_points - self.step_y); //WRONG?
            }
        } else {
            if *self.is_edge.get(self.nr_points + particle - self.step_y).unwrap() {
                running_neighbors.insert(particle - self.step_y);
            }
        }

        running_neighbors
    }

    fn describe(&self) {
        println!("Diluted two-dimensional {} by {} toroidal lattice: two adjacent points i and j in \
        the full lattice are connected by an edge with probability {}. Also known as a bond percolation.",
                 self.dim_x, self.dim_y, self.probability);
    }
}

impl DilutedLattice {
    /// Construct new diluted lattice from x-dimension, y-dimension, and probability that a certain
    /// edge is in the lattice.
    pub fn new(dim_x: usize, dim_y: usize, probability: f64, rng: ThreadRng) -> DilutedLattice {
        let bernoulli_dist = Bernoulli::new(probability).unwrap();
        let mut sampler = bernoulli_dist.sample_iter(rng);

        let mut is_edge = vec![]; // First all horizontal (x) edges, then all vertical (y) edges
        for _ in 0..2 * dim_x * dim_y { // There are 2 times as many edges as points
            is_edge.push(sampler.next().unwrap())
        }

        let step_x = 1;
        let step_y = dim_x;

        DilutedLattice {
            nr_points: dim_x * dim_y,
            dim_x,
            step_x,
            dim_y,
            step_y,
            probability,
            is_edge,
        }
    }
}