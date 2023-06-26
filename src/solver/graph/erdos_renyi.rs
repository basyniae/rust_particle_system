use std::collections::HashSet;
use crate::solver::graph::Graph;

pub struct ErdosRenyi {

}

impl Graph for ErdosRenyi {
    fn nr_points(&self) -> u64 {
        todo!()
    }

    fn get_neighbors(&self, _: u64) -> HashSet<u64> {
        todo!()
    }
}