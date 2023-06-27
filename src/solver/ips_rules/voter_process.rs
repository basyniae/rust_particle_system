use std::collections::HashSet;
use crate::{Coloration, IPSRules};

pub struct VoterProcess {
    pub nr_parties: usize,
    pub change_rate: f64,
}

impl IPSRules for VoterProcess {
    fn all_states(&self) -> Vec<usize> {
        (0..self.nr_parties).collect()
    }

    fn get_vacuum_mutation_rate(&self, _: usize, _: usize) -> f64 {
        0.0
    }

    fn get_neighbor_mutation_rate(&self, current: usize, goal: usize, sender: usize) -> f64 {
        if goal.abs_diff(sender) > 0 { // A particle cannot influence another particle to change to a party that it is not itself part off
            0.0
        } else if current.abs_diff(goal) == 0 {  // Do not influence if all particles are the same
            0.0
        } else { // Remains: current != goal == sender
            self.change_rate
        }
    }

    fn describe(&self) {
        println!("Voter process with {} parties, and change rate {}.",
                 self.nr_parties, self.change_rate)
    }
}

impl Coloration for VoterProcess {
    fn get_color(&self, state: usize) -> [u8; 4] {
        if self.nr_parties <= 10 { // From matplotlib tableau palette
            match state {
                0 => {
                    [4, 88, 147, 255] // blue
                }
                1 => {
                    [219, 97, 0, 255] // orange
                }
                2 => {
                    [16, 128, 16, 255] // green
                }
                3 => {
                    [180, 12, 13, 255] // red
                }
                4 => {
                    [116, 74, 156, 255] // purple
                }
                5 => {
                    [109, 57, 46, 255] // brown
                }
                6 => {
                    [193, 88, 160, 255] // pink
                }
                7 => {
                    [97, 97, 97, 255] // gray
                }
                8 => {
                    [154, 156, 7, 255] // olive
                }
                9 => {
                    [0, 157, 174, 255] // cyan
                }
                _ => {
                    [255, 255, 255, 255] // white
                }
            }
        } else {
            let brightness = (255.0 * state as f64 / self.nr_parties as f64).floor() as u8;
            [brightness, brightness, brightness, 255]
        }
    }
}