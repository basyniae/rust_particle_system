use std::collections::{HashMap};
use rand::seq::SliceRandom;

/// Make an initial condition of the appropriate size `grid_size` from prescribed data.
/// Fill everything with the state `fill`, except for the indices described in the hashmap
pub fn assemble_initial_condition<S: Copy>(fill: S, different: HashMap<u64, S>, grid_size: u64) -> Vec<S> {
    let mut initial_condition: Vec<S> = Vec::new();

    for i in 0..grid_size {
        initial_condition.push(
            *different.get(&i).unwrap_or(&fill)
        );
    }

    initial_condition
}

/// Make an initial condition of the appropriate size `grid_size` by sampling from a distribution.
/// A random entry from the vector `states` will be chosen. Weights can be assigned by repeating a
/// particular state in the `states` vector.
pub fn assemble_random_initial_condition<S: Copy>(states: Vec<S>, grid_size: u64) -> Vec<S> {
    let mut rng = rand::thread_rng();

    let mut initial_condition: Vec<S> = Vec::new();

    for _ in 0..grid_size {
        initial_condition.push(
            *states.choose(&mut rng).unwrap()
        );
    }

    initial_condition
}