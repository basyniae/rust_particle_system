use crate::solver::ips_rules::{IPSRules,};
use std::collections::{HashSet};
use crate::visualization::{Coloration};

// 0: Susceptible, 1: Infected
pub struct ContactProcess {
    pub death_rate: f64,
    pub birth_rate: f64,
}

impl IPSRules for ContactProcess {
    fn all_states(&self) -> HashSet<usize> {
        HashSet::from([0, 1])
    }

    fn get_vacuum_mutation_rate(&self, current: usize, goal: usize) -> f64 {
        match (current, goal) {
            (1, 0) => { self.death_rate } // death
            _ => { 0.0 }
        }
    }

    fn get_neighbor_mutation_rate(&self, current: usize, goal: usize, sender: usize) -> f64 {
        match (current, goal, sender) {
            (0, 1, 1) => { self.birth_rate } // birth
            _ => { 0.0 }
        }
    }
}

impl Coloration for ContactProcess {
    fn get_color(&self, state: usize) -> [u8; 4] {
        if state == 0 { // susceptible
            [0, 0, 0, 255]
        } else if state == 1 { // infected
            [211, 47, 47, 255]
        } else {
            panic!("State color not defined!")
        }
    }
}