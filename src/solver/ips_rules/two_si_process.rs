use crate::{Coloration, IPSRules};

// 0: no party (neutral), 1: first party, 2: second party. Parameters described in main.rs.
pub struct TwoSIProcess {
    pub birth_rate: f64,
    pub death_rate: f64,
    pub compete_rate: f64,
}

impl IPSRules for TwoSIProcess {
    fn all_states(&self) -> Vec<usize> {
        vec![0, 1, 2]
    }

    fn get_vacuum_mutation_rate(&self, current: usize, goal: usize) -> f64 {
        match (current, goal) {
            (1, 0) => { self.death_rate } // death
            (2, 0) => { self.death_rate }
            _ => { 0.0 }
        }
    }

    fn get_neighbor_mutation_rate(&self, current: usize, goal: usize, sender: usize) -> f64 {
        match (current, goal, sender) {
            (0, 1, 1) => { self.birth_rate } // birth
            (0, 2, 2) => { self.birth_rate }
            (1, 2, 2) => { self.compete_rate } // change party one to another
            (2, 1, 1) => { self.compete_rate }
            _ => { 0.0 }
        }
    }

    fn describe(&self) {
        println!("SI model with two identical invasive species (states 1 and 2), competing indirectly \
        via the available space, and directly via conversion (i.e., combat). The birth and death rates \
        for both species are {} and {} respectively, and the compete rate (a.k.a conversion rate) is \
        {}.",
                 self.birth_rate, self.death_rate, self.compete_rate)
    }
}

impl Coloration for TwoSIProcess {
    fn get_color(&self, state: usize) -> [u8; 4] {
        match state {
            0 => { [0, 0, 0, 255] }
            1 => { [180, 12, 13, 255] }
            2 => { [16, 128, 16, 255] }
            _ => { panic!("Invalid state in coloration.") }
        }
    }
}