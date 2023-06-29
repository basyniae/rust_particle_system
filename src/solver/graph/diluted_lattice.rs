use std::collections::HashSet;
use rand::distributions::{Bernoulli, Distribution};
use rand::rngs::ThreadRng;
use crate::Graph;

/// i is connected to j with probability if i,j are adjacent in the corresponding lattice
pub struct DilutedLattice {
    nr_points: u64,
    dim_x: u64,
    step_x: u64,
    dim_y: u64,
    step_y: u64,
    probability: f64,
    is_edge: Vec<bool>, // A mask over the actual lattice (which has enumerated edges)
}

impl Graph for DilutedLattice {
    fn nr_points(&self) -> u64 {
        self.nr_points
    }

    fn get_neighbors(&self, particle: u64) -> HashSet<u64> {
        let mut running_neighbors = HashSet::new();
        // There are 4 potential neighbors, which we compute as in GridND. Then we consult the is_edge list.

        // println!("Getting neighbours of {particle}");

        let x_coord = particle % self.dim_x;
        let y_coord = particle / self.dim_x;
        // println!("x: {x_coord}, y: {y_coord}");

        // the right neighbor:
        if x_coord == self.dim_x - 1 { //  if on the far boundary
            if *self.is_edge.get(particle as usize).unwrap() {
                running_neighbors.insert(particle + self.step_x - self.dim_x);
            }
        } else {
            if *self.is_edge.get(particle as usize).unwrap() {
                running_neighbors.insert(particle + self.step_x);
            }
        }
        // the left neighbor:
        if x_coord == 0 { // if on the close boundary
            if *self.is_edge.get((particle + self.dim_x - self.step_x) as usize).unwrap() {
                running_neighbors.insert(particle + self.dim_x - self.step_x);
            }
        } else {
            if *self.is_edge.get((particle - self.step_x) as usize).unwrap() {
                running_neighbors.insert(particle - self.step_x);
            }
        }

        // the bottom neighbor:
        if y_coord == self.dim_y - 1 { // if on the far boundary
            if *self.is_edge.get((self.nr_points + particle) as usize).unwrap() {
                running_neighbors.insert(particle + self.step_y - self.nr_points); //WRONG?
            }
        } else {
            if *self.is_edge.get((self.nr_points + particle) as usize).unwrap() {
                running_neighbors.insert(particle + self.step_y);
            }
        }
        // the top neighbor:
        if y_coord == 0 { // if on the close boundary
            if *self.is_edge.get((2 * self.nr_points + particle - self.step_y) as usize).unwrap() {
                running_neighbors.insert(particle + self.nr_points - self.step_y); //WRONG?
            }
        } else {
            if *self.is_edge.get((self.nr_points + particle - self.step_y) as usize).unwrap() {
                running_neighbors.insert(particle - self.step_y);
            }
        }
        // println!("The neighbours are {:?}", running_neighbors);

        running_neighbors
    }

    fn describe(&self) {
        println!("Diluted two-dimensional {} by {} toroidal lattice: two adjacent points i and j in \
        the full lattice are connected by an edge with probability {}. Also known as a bond percolation.",
                 self.dim_x, self.dim_y, self.probability);
    }
}

impl DilutedLattice {
    pub fn new(dim_x: u64, dim_y: u64, probability: f64, mut rng: ThreadRng) -> DilutedLattice {
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