use crate::solver::ips_rules::IPSRules;
use std::collections::{HashSet};
use crate::visualization::{Coloration};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum SIRProcess {
    Susceptible,
    Infected,
    Removed,
}

impl IPSRules for SIRProcess {
    fn all_states() -> HashSet<&'static Self> {
        HashSet::from([&SIRProcess::Susceptible, &SIRProcess::Infected, &SIRProcess::Removed])
    }

    fn get_vacuum_mutation_rate(self: Self, goal: Self) -> f64 {
        match (self, goal) {
            (SIRProcess::Infected, SIRProcess::Removed) => { 0.1 } // death (1.0)
            _ => { 0.0 }
        }
    }

    fn get_neighbor_mutation_rate(self: Self, goal: Self, sender: Self) -> f64 {
        match (self, goal, sender) {
            (SIRProcess::Susceptible, SIRProcess::Infected, SIRProcess::Infected) => { 0.6 } // birth (1.4)
            _ => { 0.0 }
        }
    }
}

impl Coloration for SIRProcess {
    fn get_color(self) -> [u8; 4] {
        match self {
            SIRProcess::Susceptible => { [0, 0, 0, 255] }
            SIRProcess::Infected => { [211, 47, 47, 255] }
            SIRProcess::Removed => { [100, 100, 100, 255] }
        }
    }
}