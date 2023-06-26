use std::collections::HashSet;
use crate::solver::ips_rules::IPSRules;
use crate::visualization::Coloration;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum TwoVoterProcess {
    PartyA,
    PartyB,
}

impl IPSRules for TwoVoterProcess {
    fn all_states() -> HashSet<&'static Self> {
        HashSet::from([&TwoVoterProcess::PartyA, &TwoVoterProcess::PartyB])
    }

    fn get_vacuum_mutation_rate(self: Self, _: Self) -> f64 {
        0.0 // no vacuum rate
    }

    fn get_neighbor_mutation_rate(self: Self, goal: Self, sender: Self) -> f64 {
        if self != goal && goal == sender {
            1.0
        } else {
            0.0
        }
    }
}

impl Coloration for TwoVoterProcess {
    fn get_color(self) -> [u8; 4] {
        match self {
            TwoVoterProcess::PartyA => { [255, 0, 255, 255] }
            TwoVoterProcess::PartyB => { [255, 255, 0, 255] }
        }
    }
}