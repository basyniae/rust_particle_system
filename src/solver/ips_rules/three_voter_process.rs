use std::collections::HashSet;
use crate::solver::ips_rules::IPSRules;
use crate::visualization::Coloration;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum ThreeVoterProcess {
    PartyA,
    PartyB,
    PartyC,
}

impl IPSRules for ThreeVoterProcess {
    fn all_states() -> HashSet<&'static Self> {
        HashSet::from([&ThreeVoterProcess::PartyA, &ThreeVoterProcess::PartyB, &ThreeVoterProcess::PartyC])
    }

    fn get_vacuum_mutation_rate(self: Self, _: Self) -> f64 {
        0.0 // no vacuum rate
    }

    fn get_neighbor_mutation_rate(self: Self, goal: Self, sender: Self) -> f64 {
        // symmetric rates: get influenced to the other
        if self != goal && goal == sender {
            1.0
        } else {
            0.0
        }
    }
}

impl Coloration for ThreeVoterProcess {
    fn get_color(self) -> [u8; 4] {
        match self {
            ThreeVoterProcess::PartyA => { [255, 0, 0, 255] }
            ThreeVoterProcess::PartyB => { [0, 255, 0, 255] }
            ThreeVoterProcess::PartyC => { [0, 0, 255, 255] }
        }
    }
}