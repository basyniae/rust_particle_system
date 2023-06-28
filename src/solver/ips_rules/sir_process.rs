use crate::{Coloration, IPSRules};

// 0: Susceptible, 1: Infected, 2: Removed
pub struct SIRProcess {
    pub(crate) birth_rate: f64,
    pub(crate) death_rate: f64,
}

impl IPSRules for SIRProcess {
    fn all_states(&self) -> Vec<usize> {
        vec![0, 1, 2]
    }

    fn get_vacuum_mutation_rate(&self, current: usize, goal: usize) -> f64 {
        match (current, goal) {
            (1, 2) => { self.death_rate }
            _ => { 0.0 }
        }
    }

    fn get_neighbor_mutation_rate(&self, current: usize, goal: usize, sender: usize) -> f64 {
        match (current, goal, sender) {
            (0, 1, 1) => { self.birth_rate }
            _ => { 0.0 }
        }
    }

    fn describe(&self) {
        println!("Susceptible-Infected-Removed process, with birth rate {} and death (removal) rate \
         {}", self.birth_rate, self.death_rate)
    }
}

impl Coloration for SIRProcess {
    fn get_color(&self, state: usize) -> [u8; 4] {
        match state {
            0 => { [0, 0, 0, 255] }
            1 => { [180, 12, 13, 255] }
            2 => { [97, 97, 97, 255] }
            _ => {
                panic!("State not colored!")
            }
        }
    }
}