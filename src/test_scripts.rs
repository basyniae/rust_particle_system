use std::collections::{HashMap, HashSet};
use crate::solver::graph::Graph;
use crate::solver::graph::grid_n_d::GridND;
use crate::solver::{HaltCondition, particle_system_solver, RecordCondition};
use crate::solver::assemble_initial_condition::{assemble_initial_condition, assemble_random_initial_condition};
use crate::solver::ips_rules::contact_process::ContactProcessStates;
use crate::visualization::{save_as_gif, save_as_growth_img};

pub fn hello_world() {
    println!("Hello, world!")
}

pub fn contact_process_img() {
    let nr_particles = 600;

    let initial_condition = assemble_initial_condition(
        ContactProcessStates::Susceptible,
        HashMap::from([(180, ContactProcessStates::Infected)]),
        nr_particles,
    );

    let solution = particle_system_solver(
        GridND::from(vec![nr_particles]),
        initial_condition,
        HaltCondition::TimePassed(100.0),
        RecordCondition::ConstantTime(0.1),
        rand::thread_rng(),
    );

    // println!("Solution: {:?}", solution);

    save_as_growth_img(solution, "contact_process_specimen.png", nr_particles as u32)
}

pub fn contact_process_img_random_initial() {
    let nr_particles = 600;

    let initial_condition = assemble_random_initial_condition(
        vec![ContactProcess::Susceptible, ContactProcess::Infected],
        nr_particles,
    );

    let solution = particle_system_solver(
        GridND::from(vec![nr_particles]),
        initial_condition,
        HaltCondition::TimePassed(100.0),
        RecordCondition::ConstantTime(0.1),
        rand::thread_rng(),
    );

    // println!("Solution: {:?}", solution);

    save_as_growth_img(solution, "contact_process_specimen_rnd_initial.png", nr_particles as u32)
}

pub fn sir_process_img() {
    let nr_particles = 600;

    let initial_condition = assemble_initial_condition(
        SIRProcess::Susceptible,
        HashMap::from([(180, SIRProcess::Infected)]),
        nr_particles,
    );

    let solution = particle_system_solver(
        GridND::from(vec![nr_particles]),
        initial_condition,
        HaltCondition::TimePassed(100.0),
        RecordCondition::ConstantTime(0.1),
        rand::thread_rng(),
    );

    // println!("Solution: {:?}", solution);

    save_as_growth_img(solution, "sir_process_specimen.png", nr_particles as u32)
}

pub fn sir_process_img_random_initial() {
    let nr_particles = 600;

    let initial_condition = assemble_random_initial_condition(
        vec![SIRProcess::Susceptible, SIRProcess::Susceptible, SIRProcess::Infected],
        nr_particles,
    );

    let solution = particle_system_solver(
        GridND::from(vec![nr_particles]),
        initial_condition,
        HaltCondition::TimePassed(100.0),
        RecordCondition::ConstantTime(0.1),
        rand::thread_rng(),
    );

    // println!("Solution: {:?}", solution);

    save_as_growth_img(solution, "sir_process_specimen_rnd_initial.png", nr_particles as u32)
}

pub fn contact_process_gif() {
    let (x_size, y_size) = (10, 10);

    let initial_condition = assemble_initial_condition(
        ContactProcess::Susceptible,
        HashMap::from([(55, ContactProcess::Infected)]),
        x_size * y_size,
    );

    let solution = particle_system_solver(
        GridND::from(vec![x_size, y_size]),
        initial_condition,
        HaltCondition::TimePassed(100.0),
        RecordCondition::ConstantTime(10.0),
        rand::thread_rng(),
    );

    println!("bing! length: {}", solution.len());

    save_as_gif(solution, "contact_process_gif.gif", x_size as u32, y_size as u32, 17);
}

pub fn sir_process_gif() {
    let (x_size, y_size) = (40, 40);

    let initial_condition = assemble_initial_condition(
        SIRProcess::Susceptible,
        HashMap::from([(55, SIRProcess::Infected)]),
        x_size * y_size,
    );

    let solution = particle_system_solver(
        GridND::from(vec![x_size, y_size]),
        initial_condition,
        HaltCondition::TimePassed(100.0),
        RecordCondition::ConstantTime(0.1),
        rand::thread_rng(),
    );

    println!("bing! length: {}", solution.len());

    save_as_gif(solution, "sir_process_gif.gif", x_size as u32, y_size as u32, 100);
}

pub fn voter_process_img() {
    let nr_particles = 600;

    let initial_condition = assemble_random_initial_condition(
        vec![TwoVoterProcess::PartyA, TwoVoterProcess::PartyB],
        nr_particles,
    );

    let solution = particle_system_solver(
        GridND::from(vec![nr_particles]),
        initial_condition,
        HaltCondition::TimePassed(100.0),
        RecordCondition::ConstantTime(0.1),
        rand::thread_rng(),
    );

    // println!("Solution: {:?}", solution);

    save_as_growth_img(solution, "voter_process_specimen.png", nr_particles as u32)
}

pub fn voter_process_gif() {
    let (x_size, y_size) = (5, 5); //  Compiled, the size (240, 120) takes ~60 seconds to compute

    let initial_condition = assemble_random_initial_condition(
        vec![TwoVoterProcess::PartyA, TwoVoterProcess::PartyB],
        x_size * y_size,
    );

    let solution = particle_system_solver(
        GridND::from(vec![x_size, y_size]),
        initial_condition,
        HaltCondition::TimePassed(100.0),
        RecordCondition::ConstantTime(0.5),
        rand::thread_rng(),
    );

    println!("bing! length: {}", solution.len());

    save_as_gif(solution, "voter_process_gif.gif", x_size as u32, y_size as u32, 100);
}

pub fn three_voter_process_img() {
    let nr_particles = 600;

    let initial_condition = assemble_random_initial_condition(
        vec![ThreeVoterProcess::PartyA, ThreeVoterProcess::PartyB, ThreeVoterProcess::PartyC],
        nr_particles,
    );

    let solution = particle_system_solver(
        GridND::from(vec![nr_particles]),
        initial_condition,
        HaltCondition::TimePassed(100.0),
        RecordCondition::ConstantTime(0.1),
        rand::thread_rng(),
    );

    save_as_growth_img(solution, "three_voter_process_specimen.png", nr_particles as u32)
}

pub fn graph_tester() {
    /// A function defined on all objects which implement the Graph trait
    fn get_neighs_of_four<G: Graph>(gra: G) -> HashSet<u64> {
        gra.get_neighbors(12)
    }

    let g = GridND::from(
        (
            vec![5, 5, 5], vec![true, true, true]
        )
    );

    println!("{:?}", g);

    println!("{:?}", g.get_neighbors(62)); // center of a 5x5x5 cube
}