use crate::solver::ips_rules::{IPSRules, IPSStates};
use std::collections::{HashSet};
use crate::visualization::{Coloration};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum ContactProcessStates {
    Susceptible,
    Infected,
}

impl IPSStates for ContactProcessStates {}

pub struct ContactProcessRules {
    pub birth_rate: f64,
    pub death_rate: f64,
}

impl IPSRules<ContactProcessStates> for ContactProcessRules {
    fn all_states(&self) -> HashSet<&'static ContactProcessStates> {
        HashSet::from([&ContactProcessStates::Susceptible, &ContactProcessStates::Infected])
    }

    fn get_vacuum_mutation_rate(self, current: ContactProcessStates, goal: ContactProcessStates) -> f64 {
        match (current, goal) {
            (ContactProcessStates::Infected,
                ContactProcessStates::Susceptible) => { self.birth_rate }
            _ => { 0.0 }
        }
    }

    fn get_neighbor_mutation_rate(self, current: ContactProcessStates, goal: ContactProcessStates, sender: ContactProcessStates) -> f64 {
        match (current, goal, sender) {
            (ContactProcessStates::Susceptible,
                ContactProcessStates::Infected,
                ContactProcessStates::Infected) => { self.death_rate }
            _ => { 0.0 }
        }
    }
}

impl Coloration for ContactProcessStates {
    fn get_color(self) -> [u8; 4] {
        match self {
            ContactProcessStates::Susceptible => { [0, 0, 0, 255] }
            ContactProcessStates::Infected => { [211, 47, 47, 255] }
        }
    }
}