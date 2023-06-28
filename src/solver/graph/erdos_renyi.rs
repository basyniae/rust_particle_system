use std::collections::HashSet;
use rand::distributions::{Bernoulli, Distribution};
use rand::rngs::ThreadRng;
use crate::solver::graph::Graph;


pub struct ErdosRenyi {
    cliques: Vec<HashSet<u64>>,
    // The order of the cliques does not matter, but we're only checking if x is a member of each clique. So that has to be a hashset.
    nr_points: u64,
    parameter: f64,
}

impl Graph for ErdosRenyi {
    fn nr_points(&self) -> u64 {
        self.nr_points
    }

    fn get_neighbors(&self, particle: u64) -> HashSet<u64> {
        let mut running_neighbours: HashSet<u64> = HashSet::new();
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
        println!("Erdos-Renyi graph: two different points i and j are by an edge with probability {}",
                 self.parameter);
        // println!("{:?}", self.cliques)
    }
}

impl ErdosRenyi {
    pub fn new(nr_points: u64, parameter: f64, mut rng: ThreadRng) -> ErdosRenyi {
        let bernoulli_dist = Bernoulli::new(parameter).unwrap();

        let mut cliques: Vec<HashSet<u64>> = vec![];

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
            parameter,
        }
    }
}