use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use rand::distributions::{WeightedError, WeightedIndex, Distribution};
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::solver::exponential_distribution::StandardExponential;
use crate::solver::graph::Graph;
use crate::solver::ips_rules::{IPSRules,};

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
/// # Variants
/// * `ConstantTime(f64)`: record the state after a constant amount of time has passed.
/// * `EveryNthStep(usize)`: record the state every nth step. Useful for discrete-time particle
/// systems.
/// * `Final()`: only record the final state.
#[derive(Debug)]
pub enum RecordCondition {
    ConstantTime(f64),
    EveryNthStep(usize),
    Final(),
}

impl RecordCondition {
    /// Given the record condition `self`, how often should the previous state be recorded?
    pub fn how_often_record(&self, time_passed: f64, time_step: f64, steps_taken: u64, is_final: bool) -> usize {
        match self {
            RecordCondition::ConstantTime(time_interval) => {
                ((time_passed / time_interval).floor() - ((time_passed - time_step) / time_interval).floor())
                    as usize
            }
            RecordCondition::EveryNthStep(n) => {
                ((steps_taken as usize) % n == 0) as usize
            }
            RecordCondition::Final() => {
                if is_final {
                    1
                } else {
                    0
                }
            }
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
/// represented by integers, if applicable, 0 is the default state.
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
/// initial condition. Record the state every 0.5 time units. Write the output to a 40x40 gif, where
/// every frame takes 100 ms.
/// ```
/// // make the initial condition
/// let initial_condition = assemble_random_initial_condition(
///     vec![TwoVoterProcess::PartyA, TwoVoterProcess::PartyB],
///     40 * 40,
/// );
///
/// // make the particle system
/// let ips_rules = Box::new(
///         Two
///
/// // run the simulation TODO: Update example
/// let solution = particle_system_solver(
///     Box::new(
///
///     ),
///     GridND::from(vec![40, 40]),
///     initial_condition,
///     HaltCondition::TimePassed(100.0),
///     RecordCondition::ConstantTime(0.5),
///     rand::thread_rng(),
/// );
///
/// // put the output into a pretty gif
/// save_as_gif(solution, "voter_process.gif", 40, 40, 100)
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

    // Initialize integer-to-state mapping
    let mut int_to_state: Vec<usize> = vec![];
    for s in ips_rules.all_states() {
        int_to_state.push(s.clone())
    }

    // Initialize state & reactivity vectors
    let mut states: Vec<usize> = initial_condition;
    assert_eq!(states.len(), graph.nr_points() as usize); // Check if enough information was given in the initial state

    let mut reactivities: Vec<f64> = Vec::with_capacity(graph.nr_points() as usize);

    // Compute initial rates
    for i in 0..graph.nr_points() { // Loop over all states
        // Count how many of which neighbor their are, by looping over the neighbors
        let mut neigh_counts: HashMap<usize, u64> = HashMap::new();
        for j in graph.get_neighbors(i) {
            let state_j = states.get(j as usize).unwrap();
            neigh_counts.insert(
                state_j.clone(),
                neigh_counts.get(state_j).unwrap_or(&0u64) + 1,
            );
        }
        // Pass these counts to the IPS rules object to find the rate
        reactivities.push(
            ips_rules.get_reactivity(states[i as usize].clone(), &neigh_counts)
        );
    }

    // Initialize the total rate
    let mut total_reactivity: f64 = reactivities.iter().sum();
    // Initialize state record
    let mut states_record = states.clone();

    // Initialize timekeeping
    let mut time_passed = 0.0;
    let mut steps_recorded = 1;
    let mut steps_taken = 0;

    // Initialize location-finding distribution
    let mut distr_location = match WeightedIndex::new(&reactivities) {
        Ok(distribution) => distribution,
        Err(e) => {
            println!("The rates are {:?}", reactivities);
            panic!("Problem assembling location distribution: {:?}", e)
        }
    };

    // * PHASE 2: Simulation loop * //
    while halting_condition.should_continue(time_passed, steps_recorded, steps_taken) {
        steps_taken += 1;
        let prev_state = states.clone();

        // Generate time step (until next event)
        let time_step: f64 = {
            let standard_exp_object: StandardExponential = rng.gen();
            standard_exp_object.0 / total_reactivity
        };

        time_passed += time_step;

        // Find place where update occurs
        let update_location = distr_location.sample(&mut rng) as u64; // Sample the distribution

        // Find out to which state the selected particle transitions
        // Figure out neighbors and their states
        let neighs: HashSet<u64> = graph.get_neighbors(update_location as u64);
        let mut neigh_state_counts: HashMap<usize, u64> = HashMap::new();

        for j in &neighs {
            let state_j = states.get(*j as usize).unwrap();
            neigh_state_counts.insert(
                state_j.clone(),
                neigh_state_counts.get(state_j).unwrap_or(&0u64) + 1,
            );
        }

        // Assemble transition rate distribution (by sampling all states)
        let mut change_rates: Vec<f64> = vec![];
        for to_state in ips_rules.all_states() {
            change_rates.push(
                ips_rules.get_mutation_rate(states[update_location as usize],
                                            to_state.clone(),
                                            &neigh_state_counts));
        }

        let distr_to_state = match WeightedIndex::new(change_rates) {
            Ok(distribution) => { distribution }
            Err(WeightedError::AllWeightsZero) => { break }
            Err(other) => {panic!("Strange error! {:?}", other)}
        };

        // Sample the distribution we found to get the state to which the particle transitions
        let new_state = int_to_state[distr_to_state.sample(&mut rng) as usize].clone();

        // Record previous state our particle was in
        let old_particle_state = (*states.get(update_location as usize).unwrap()).clone();
        // Change old state to new state
        states[update_location as usize] = new_state.clone();

        // Compute own new rate
        // first need the state counts of the neighbors
        let mut neigh_state_counts: HashMap<usize, u64> = HashMap::new();
        for n in &neighs {
            neigh_state_counts.insert(
                (*states.get(*n as usize).unwrap()).clone(),
                neigh_state_counts.get(&states[*n as usize]).unwrap_or(&0) + 1,
            );
        }
        total_reactivity -= reactivities[update_location as usize]; // Need to update total rate as well
        reactivities[update_location as usize] = ips_rules.get_reactivity(new_state, &neigh_state_counts);
        total_reactivity += reactivities[update_location as usize];


        // Update surrounding rates & total rate
        for n in &neighs { // for every neighbor of the particle that's being updated
            // Subtract the old spread rate //After some steps we get a floating point error. Doesn't matter, it's linear, but it does panic at negative rates

            let old_spread_rate = ips_rules.get_neighbor_reactivity(states[*n as usize], old_particle_state.clone());
            reactivities[*n as usize] -= old_spread_rate;
            total_reactivity -= old_spread_rate; // (we'll hopefully never get floating point problems with this?)
            // and add the new spread rate
            let new_spread_rate = ips_rules.get_neighbor_reactivity(states[*n as usize], new_state.clone());
            reactivities[*n as usize] += new_spread_rate;
            total_reactivity += new_spread_rate;
            if reactivities[*n as usize] < 0.0 { // floating point error safety net
                reactivities[*n as usize] = 0.0;
            }
        }

        // Update rates for selecting the next point
        let mut changing_weights = vec![(update_location as usize, reactivities.get(update_location as usize).unwrap())]; // harvest the new rate of the updating particle
        for n in &neighs { // harvest the changed rates from the neighbors
            changing_weights.push((*n as usize, reactivities.get(*n as usize).unwrap()));
        }
        changing_weights.sort_by(|a, b| (a.0).cmp(&b.0)); // sorting is required for .update_weights()
        match distr_location.update_weights(&changing_weights[..]) {
            Ok(_) => {}
            Err(WeightedError::AllWeightsZero) => { break; } // All particles have died, no more reaction is possible
            Err(e) => { panic!("Changing weights: {:?}, Error: {}", changing_weights, e) }
        }; // By far the heaviest operation in the whole program

        // Record new state
        for _ in 0..record_condition.how_often_record(time_passed, time_step, steps_taken, false) {
            states_record.append(&mut prev_state.clone());
            steps_recorded += 1;
            if !halting_condition.should_continue(time_passed, steps_recorded, steps_taken) { // we want to check the halting condition each step
                break;
            }
        }
    }

    // Record final state (if that record condition is given)
    for _ in 0..record_condition.how_often_record(time_passed, 0.0, steps_taken, true) {
        states_record.append(&mut states.clone());
    }

    (states_record, states, time_passed, steps_recorded, steps_taken)
}
