use std::collections::{HashMap, HashSet};
use crate::solver::ips_rules::contact_process::{ContactProcessRules, ContactProcessStates};

pub mod contact_process;

/// Trait encoding the rules for the evolution of an interacting particle system.
/// To be implemented on an enum.
///
/// Overwrite the following functions for each interacting particle system:
/// * `all_states`
/// * `get_vacuum_mutation_rate`
/// * `get_neighbor_mutation_rate`
///
/// The word `reactivity` is reserved for transition of one state to any other state, meaning the
/// rate at which any update occurs. The word `mutation` is reserved for transition of one state
/// to a particular other state.
pub trait IPSRules<T: IPSStates + Copy + 'static> {
    /// Return a hash set of all the states in the system, i.e., all variants of the enum..
    ///
    /// Overwrite for each system.
    fn all_states(&self) -> HashSet<&'static T>;

    /// Returns the rate at which a particle in a given state `self` changes to the state `goal`
    /// in vacuum, meaning without any neighbors influencing it.
    ///
    /// Overwrite for each system.
    ///
    /// # Example
    /// In the contact process, there is a chance that the virus in a particular infected particle
    /// dies, so that this particles state becomes susceptible. Say this happens at rate 1.4.
    /// We will get
    /// `Infected.get_vacuum_change_rate(Susceptible) = 1.4`. All other combinations of susceptible
    /// and infected in the parameters should return 0.0 (no spontaneous change).
    fn get_vacuum_mutation_rate(self, current: T, goal: T) -> f64;


    /// Returns increase the rate at which a particular particle in a given state `self` changes to
    /// a state `goal` due to the presence of a neighbor in the state `sender`.
    ///
    /// Overwrite for each system.
    ///
    /// # Examples
    /// * In the contact process, the presence of every infected neighbor raises the rate of a susceptible
    /// particle becoming infected. Say the rate is raised by 1.0. Hence we will get
    /// `Susceptible.get_received_change_rate_from_sender(Infected, Infected) = 1.0`. All other
    /// combinations of susceptible and infected in the parameters should return 0.0 (no influence).
    ///
    /// * In the voter process, the presence of party A neighbours increases the rates of a party B
    /// particle changing to party A.
    fn get_neighbor_mutation_rate(self, current: T, goal: T, sender: T) -> f64;

    /// Returns the increase in rate at which a particle in a given state `self` changes to any
    /// other state due to the presence of a neighbors in the state `sender`.
    ///
    /// Do not overwrite, the default implementation is correct.
    fn get_neighbor_reactivity(self, current: T, sender: T) -> f64 where T: 'static, Self: Sized {
        let mut running_rate = 0.0;

        for other in self.all_states() {
            running_rate += self.get_neighbor_mutation_rate(current, *other, sender)
        }

        running_rate
    }

    /// Returns the rate at which a particle in a given state `self` changes to any other state due
    /// to the influence of all of its neighbors.
    ///
    /// Do not overwrite, the default implementation is correct.
    fn get_reactivity(self, current: T, neighbor_counts: &HashMap<T, u64>) -> f64 where T: 'static, Self: Sized {
        let mut running_rate = 0.0;

        // Condition over to which state `goal` self will transition
        for goal in self.all_states() {
            // Each goal has an associated vacuum rate
            running_rate += self.get_vacuum_mutation_rate(current, *goal);
            // as well as a contribution due to the neighbors, which depends on how many of
            // which neighbor there are.
            for (neigh_state, neigh_count) in neighbor_counts.into_iter() {
                running_rate += (*neigh_count as f64) * self.get_neighbor_mutation_rate(current, *goal, *neigh_state)
            }
        }

        running_rate
    }

    /// Returns the rate at which a particle in a given state `self` changes to a particular state
    /// `other` due to the influence of all of its neighbors.
    ///
    /// Do not overwrite, the default implementation is correct.
    fn get_mutation_rate(self, current: T, goal: T, neighbor_counts: &HashMap<T, u64>) -> f64 where T: 'static, Self: Sized {
        // Start with the vacuum rate of changing self to goal
        let mut running_rate = self.get_vacuum_mutation_rate(current, goal);
        // Then add the influence of all neighbors.
        for (neigh_state, neigh_count) in neighbor_counts.into_iter() {
            running_rate += (*neigh_count as f64) * self.get_neighbor_mutation_rate(current, goal, *neigh_state)
        }

        running_rate
    }
}

pub trait IPSStates {}

pub enum IPSKind {
    SIRProcess,
    ContactProcess,
    TwoVoterProcess,
    ThreeVoterProcess,
}

pub fn ips_rules_constructor(ips_kind: IPSKind, ips_parameters: Vec<f64>) -> Box<dyn IPSRules<dyn IPSRules>> {
    match ips_kind {
        IPSKind::SIRProcess => {
            todo!()
        }

        IPSKind::ContactProcess => {
            assert!(ips_parameters.len() == 2);

            Box::new(ContactProcessRules {
                birth_rate: *ips_parameters.get(0).unwrap(),
                death_rate: *ips_parameters.get(1).unwrap(),
            })
        }

        IPSKind::TwoVoterProcess => {
            todo!()
        }

        IPSKind::ThreeVoterProcess => {
            todo!()
        }
    }
}