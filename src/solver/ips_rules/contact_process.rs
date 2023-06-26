use crate::solver::ips_rules::IPSRules;
use std::collections::{HashSet};
use crate::visualization::{Coloration};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum ContactProcess {
    Susceptible,
    Infected,
}

impl IPSRules for ContactProcess {
    fn all_states() -> HashSet<&'static Self> {
        HashSet::from([&ContactProcess::Susceptible, &ContactProcess::Infected])
    }

    fn get_vacuum_mutation_rate(self: Self, goal: Self) -> f64 {
        match (self, goal) {
            (ContactProcess::Infected, ContactProcess::Susceptible) => { 1.0 } // death (1.0)
            _ => { 0.0 }
        }
    }

    fn get_neighbor_mutation_rate(self: Self, goal: Self, sender: Self) -> f64 {
        match (self, goal, sender) {
            (ContactProcess::Susceptible, ContactProcess::Infected, ContactProcess::Infected) => { 1.4 } // birth (1.4)
            _ => { 0.0 }
        }
    }
}

impl Coloration for ContactProcess {
    fn get_color(self) -> [u8; 4] {
        match self {
            ContactProcess::Susceptible => { [0, 0, 0, 255] }
            ContactProcess::Infected => { [211, 47, 47, 255] }
        }
    }
}