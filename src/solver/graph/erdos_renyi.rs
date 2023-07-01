use std::collections::HashSet;
use rand::distributions::{Bernoulli, Distribution};
use rand::rngs::ThreadRng;
use crate::solver::graph::Graph;


pub struct ErdosRenyi {
    // The description of an E-R graph is not unique given a list of cliques. This leaves room for
    // an optimization with less, bigger cliques.
    cliques: Vec<HashSet<usize>>,
    // The order of the cliques does not matter, but we're only checking if x is a member of each clique. So that has to be a hashset.
    nr_points: usize,
    probability: f64,
}

impl Graph for ErdosRenyi {
    fn nr_points(&self) -> usize {
        self.nr_points
    }

    fn get_neighbors(&self, particle: usize) -> HashSet<usize> {
        let mut running_neighbours: HashSet<usize> = HashSet::new();
        for clique in self.cliques.iter() {
            if clique.contains(&particle) {
                for neigh in clique {
                    if neigh != &particle {
                        running_neighbours.insert(neigh.clone());
                    }
                }
            }
        }

        running_neighbours.remove(&particle);
        running_neighbours
    }

    fn describe(&self) {
        println!("Erdos-Renyi graph: two different points i and j are connected by an edge with \
        probability {}",
                 self.probability);
    }
}

impl ErdosRenyi {
    pub fn new(nr_points: usize, probability: f64, mut rng: ThreadRng) -> ErdosRenyi {
        let bernoulli_dist = Bernoulli::new(probability).unwrap();

        let mut cliques: Vec<HashSet<usize>> = vec![];

        // Loop over all unordered pairs of points, and determine randomly if they're connected
        for i in 0..nr_points {
            for j in 0..i {
                if bernoulli_dist.sample(&mut rng) {
                    cliques.push(HashSet::from([i, j]))
                }
            }
        }

        ErdosRenyi {
            cliques,
            nr_points,
            probability,
        }
    }
}