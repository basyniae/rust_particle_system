use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

use rand::distributions::{Distribution, WeightedError, WeightedIndex};
use rand::Rng;
use rand::rngs::ThreadRng;

use crate::solver::exponential_distribution::StandardExponential;
use crate::solver::graph::Graph;
use crate::solver::ips_rules::IPSRules;

pub mod ips_rules;
pub mod graph;
pub mod assemble_initial_condition;

mod exponential_distribution;

/// Enum to be passed into `particle_system_solver` that determines the simulation halting
/// condition. Implements `HaltCondition::should_continue`.
#[derive(Debug)]
pub enum HaltCondition {
    /// Stop the simulation after a certain amount of time has passed. Physical in
    /// the sense that an experiment took this amount of time.
    TimePassed(f64),
    /// Stop the simulation after a certain amount of steps have been recorded.
    /// Useful for discrete-time particle systems.
    StepsRecorded(u64),
    /// Stop the simulation after a certain amount of steps have been taken.
    /// Useful for discrete-time particle systems.
    StepsTaken(u64),
}

impl HaltCondition {
    /// Given the halting condition `self`, should the simulation continue given all the parameters
    /// of the current state of the simulation?
    pub fn should_continue(&self, time_passed: f64, steps_recorded: u64, steps_taken: u64) -> bool {
        return match self {
            HaltCondition::TimePassed(limit) => {
                time_passed < *limit
            }
            HaltCondition::StepsRecorded(limit) => {
                steps_recorded < *limit
            }
            HaltCondition::StepsTaken(limit) => {
                steps_taken <= *limit
            }
        };
    }
}

/// Enum to be passed into `particle_system_solver` that determines the recording condition.
#[derive(Debug)]
pub enum RecordCondition {
    /// Record the state after a constant amount of time has passed.
    ConstantTime(f64),
    /// Record the state every nth step. Useful for discrete-time particle systems.
    EveryNthStep(usize),
    /// Only record the final state.
    Final(),
}

impl RecordCondition {
    /// Given the record condition `self`, how often should the previous state be recorded?
    /// Called at the end of every step.
    pub fn how_often_record(&self, time_passed: f64, time_step: f64, steps_taken: u64) -> usize {
        match self {
            RecordCondition::ConstantTime(time_interval) => {
                ((time_passed / time_interval).floor() - ((time_passed - time_step) / time_interval).floor())
                    as usize
            }
            RecordCondition::EveryNthStep(n) => {
                ((steps_taken as usize) % n == 0) as usize
            }
            RecordCondition::Final() => { 0 }
        }
    }
}

/// Interacting particle system simulator. The inputs define a particular particle system, the
/// output is a record of how that particular particle system might develop (note that this is
/// nondeterministic).
///
/// # Parameters
/// * `ips_rules`: Defines the evolution rules of the interaction particle system.
/// * `graph`: Graph which defines neighboring states (e.g., line, circle, torus, GridND). Has to
/// implement `Graph` trait.
/// * `initial_condition`: Vector containing the initial states of the particles. States are
/// represented by integers. If applicable, 0 is the default state.
/// * `halting_condition`: HaltCondition enum which determines under what conditions the simulation
/// halts (e.g., stop after 10.0 time units, or 20 steps have been recorded).
/// * `record_condition`: RecordCondition enum which determines under what conditions the state
/// of the simulation is recorded into the output (e.g., record every step, record every 1.0 time
/// unit).
/// * `rng`: ThreadRng input. Most likely you want to input `rand::thread_rng()`.
///
/// # Outputs
/// A tuple consisting of
/// * A vector which contains snapshots of the particle system at different times. If `n` steps have
/// been recorded of a system with `x` particles, the length of the output vector is `nx`. The `i`th
/// snapshot (`0 <= i <= n`) can be found at indices `ix` to `(i+1)x-1`.
/// * A vector containing only the final state, in the format above.
/// * The total simulated time (f64)
/// * The total number of steps recorded (u64)
/// * The total number of steps simulated (u64)
///
/// # Example
/// Simulate the two voter process for 100.0 time units on a 40x40 toroidal grid, with random
/// initial condition. Record the state every 0.1 time units. Write the output to a 40x40 gif, where
/// every frame takes 20 ms (50 fps).
/// ```
/// // make graph
/// let graph = Box::new(GridND::from((vec![40, 40])));
///
/// // make ips rules
/// let ips_rules = Box::new(VoterProcess {
///     nr_parties: 2,
///     change_rate: 1.0,
/// });
///
/// // make the initial condition
/// let initial_condition = assemble_random_initial_condition(
///     vec![0, 1],
///     40 * 40,
/// );
///
///
/// // run the simulation
/// let solution = particle_system_solver(
///     ips_rules,
///     graph,
///     initial_condition,
///     HaltCondition::TimePassed(100.0),
///     RecordCondition::ConstantTime(0.1),
///     rand::thread_rng(),
/// );
///
/// // put the output into a pretty gif
/// save_as_gif(solution, "voter_process.gif", 40, 40, 20)
/// ```
pub fn particle_system_solver(
    ips_rules: Box<dyn IPSRules>,
    graph: Box<dyn Graph>,
    initial_condition: Vec<usize>,
    halting_condition: HaltCondition,
    record_condition: RecordCondition,
    mut rng: ThreadRng,
) -> (Vec<usize>, Vec<usize>, f64, u64, u64) {
    // * PHASE I: Initialization * //

    // Initialize state & reactivity vectors
    let mut states: Vec<usize> = initial_condition;

    // Check if enough information was given in the initial state
    assert_eq!(states.len(), graph.nr_points());

    // Compute initial reactivities
    let mut reactivities: Vec<f64> = Vec::with_capacity(graph.nr_points());

    for i in 0..graph.nr_points() { // Loop over all points
        // Count how many of which neighboring states this point i has, by looping over all neighbors
        let mut neigh_counts: HashMap<usize, usize> = HashMap::new();

        for j in graph.get_neighbors(i) {
            let state_j = states.get(j).unwrap();
            neigh_counts.insert(
                state_j.clone(),
                neigh_counts.get(state_j).unwrap_or(&0usize) + 1,
            );
        }

        // Pass these counts to the IPS rules object to find the rate
        reactivities.push(
            ips_rules.get_reactivity(states[i].clone(), &neigh_counts)
        );
    }

    // Initialize the total rate
    let mut total_reactivity: f64 = reactivities.iter().sum();
    // Initialize state record
    let mut states_record: Vec<usize> = vec![];

    // Initialize timekeeping
    let mut time_passed = 0.0;
    let mut steps_recorded = 1;
    let mut steps_taken = 0;

    // Initialize location-finding distribution
    let mut distr_location = match WeightedIndex::new(&reactivities) {
        Ok(distribution) => distribution,
        Err(e) => {
            // Debug information
            println!("The states are {:?}", states);
            println!("The rates are {:?}", reactivities);
            panic!("Problem assembling location distribution: {:?}", e)
        }
    };

    // * PHASE 2: Simulation loop * //
    while halting_condition.should_continue(time_passed, steps_recorded, steps_taken) {
        /* Update timekeeping */
        steps_taken += 1;
        let prev_state = states.clone();

        // Generate time step (until next event)
        let time_step: f64 = {
            let standard_exp_object: StandardExponential = rng.gen();
            standard_exp_object.0 / total_reactivity
        };

        time_passed += time_step;

        /* Find place where update occurs */
        // Sample the distribution
        let update_location = distr_location.sample(&mut rng);

        /* Find out to which state the selected particle transitions */
        // Figure out neighbors and their states
        let neighs: HashSet<usize> = graph.get_neighbors(update_location);
        let mut neigh_state_counts: HashMap<usize, usize> = HashMap::new();

        for j in &neighs {
            let state_j = states.get(*j).unwrap();
            neigh_state_counts.insert(
                state_j.clone(),
                neigh_state_counts.get(state_j).unwrap_or(&0usize) + 1,
            );
        }

        // Assemble transition rate distribution (by sampling all states)
        let mut change_rates: Vec<f64> = vec![];
        for to_state in ips_rules.all_states() {
            change_rates.push(
                ips_rules.get_mutation_rate(states[update_location],
                                            to_state.clone(),
                                            &neigh_state_counts));
        }

        // Initialize distribution object
        let distr_to_state = match WeightedIndex::new(change_rates) {
            Ok(distribution) => { distribution }
            Err(WeightedError::AllWeightsZero) => { break; }
            Err(other) => { panic!("Strange error! {:?}", other) }
        };

        // Sample the distribution we found to get the state to which the particle transitions
        let new_state = distr_to_state.sample(&mut rng);

        /* Update states and reactivities */

        // Record previous state our particle was in
        let old_particle_state = (*states.get(update_location).unwrap()).clone();
        // Change old state to new state
        states[update_location] = new_state.clone();

        // Compute own new rate
        // first need the state counts of the neighbors
        let mut neigh_state_counts: HashMap<usize, usize> = HashMap::new();
        for n in &neighs {
            neigh_state_counts.insert(
                (*states.get(*n).unwrap()).clone(),
                neigh_state_counts.get(&states[*n]).unwrap_or(&0) + 1,
            );
        }
        total_reactivity -= reactivities[update_location]; // Need to update total rate as well
        reactivities[update_location] = ips_rules.get_reactivity(new_state, &neigh_state_counts);
        total_reactivity += reactivities[update_location];



        // Update surrounding rates & total rate
        for n in &neighs {
            // For every neighbor of the particle that's being updated

            // Compute the old spread rate
            let old_spread_rate = ips_rules.get_neighbor_reactivity(states[*n], old_particle_state.clone());
            // Subtract the old spread rate from both the reactivities and the total reactivity
            reactivities[*n] -= old_spread_rate;
            total_reactivity -= old_spread_rate;
            // Compute the new spread rate
            let new_spread_rate = ips_rules.get_neighbor_reactivity(states[*n], new_state.clone());
            // Add the new spread rate to both the reactivities and total reactivity
            reactivities[*n] += new_spread_rate;
            total_reactivity += new_spread_rate;

            // Floating point error safety net, WeightIndex panics at negative values
            if reactivities[*n] < 0.0 {
                reactivities[*n] = 0.0;
            }

        }

        // Update rates for selecting the next point
        // By finding all the points at which the reactivity changes.
        // Collect a list of reactivities that change.
        // TODO: This is ugly, and I want to get rid of it, but I'm not sure how to work around the references. May be able to get rid of `reactivities` entirely
        let mut changing_weights = vec![(update_location, reactivities.get(update_location).unwrap())]; // harvest the new rate of the updating particle
        for n in &neighs { // harvest the changed rates from the neighbors
            changing_weights.push((*n, &reactivities[*n]));
        }
        changing_weights.sort_by(|a, b| (a.0).cmp(&b.0)); // sorting is required for .update_weights()
        match distr_location.update_weights(&changing_weights[..]) {
            Ok(_) => {}
            Err(WeightedError::AllWeightsZero) => { break; } // All particles have died, no more reaction is possible
            Err(e) => { panic!("Changing weights: {:?}, Error: {}", changing_weights, e) }
        }; // By far the heaviest operation in the whole program

        // Record new state
        for _ in 0..record_condition.how_often_record(time_passed, time_step, steps_taken) {
            states_record.append(&mut prev_state.clone());
            steps_recorded += 1;
            if !halting_condition.should_continue(time_passed, steps_recorded, steps_taken) { // we want to check the halting condition each step
                break;
            }
        }
    }

    // * PHASE III: Cleanup * //

    // Record final state
    states_record.append(&mut states.clone());

    (states_record, states, time_passed, steps_recorded, steps_taken)
}
