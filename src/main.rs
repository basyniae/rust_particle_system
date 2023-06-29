use std::collections::{HashMap, HashSet};
use std::time::Instant;
use clap::{arg, ArgGroup, command, value_parser};
use crate::solver::assemble_initial_condition::{assemble_initial_condition, assemble_random_initial_condition};
use crate::solver::graph::{Graph};
use crate::solver::ips_rules::{IPSRules};
use crate::solver::{HaltCondition, particle_system_solver, RecordCondition};
use crate::solver::graph::diluted_lattice::DilutedLattice;
use crate::solver::graph::erdos_renyi::ErdosRenyi;
use crate::solver::graph::grid_n_d::GridND;
use crate::solver::ips_rules::si_process::SIProcess;
use crate::solver::ips_rules::sir_process::SIRProcess;
use crate::solver::ips_rules::two_si_process::TwoSIProcess;
use crate::solver::ips_rules::voter_process::VoterProcess;
use crate::visualization::{Coloration, save_as_gif, save_as_growth_img};

pub mod visualization;
pub mod solver;

fn main() {
    let matches = command!("cmd")
        // Select graph
        .arg(arg!(--"graph-grid-nd" <DIMENSIONS>).required(false)
            .help("Run particle system on an n-dimensional grid. Specify dimensions.")
            .min_values(1)
            .multiple_values(true)
            .value_parser(value_parser!(u64))
            .validator(|s| s.parse::<u64>()))
        .arg(arg!(--"graph-erdos-renyi" <DIMENSIONS_AND_COUNT>).required(false)
            .help("Run particle system on an Erdos-Renyi graph. Specify dimensions and average \
            number of neighbours per particle.")
            .min_values(2)
            .max_values(2)
            .value_parser(value_parser!(u64))
            .validator(|s| s.parse::<u64>())
            .multiple_values(true))
        .arg(arg!(--"graph-diluted-lattice" <X_AND_Y_DIMENSIONS_AND_PERCENTILE>).required(false)
            .help("Run particle system on a 2d diluted lattice graph. Specify dimensions and \
            percentile of the edges being present in the diluted lattice. (100% corresponds with \
            the ordinary lattice.)")
            .min_values(3)
            .max_values(3)
            .value_parser(value_parser!(u64))
            .validator(|s| s.parse::<u64>())
            .multiple_values(true))
        .group(ArgGroup::new("graph-kind")
            .args(&["graph-grid-nd", "graph-erdos-renyi", "graph-diluted-lattice"])
            .required(true)
        )
        // Select IPS
        .arg(arg!(--"ips-si" <BIRTH_AND_DEATH_RATE>).required(false)
            .help("Susceptible-Infected (aka contact) process, specify birth and death rates.")
            .min_values(2)
            .max_values(2)
            .value_parser(value_parser!(f64))
            .validator(|s| s.parse::<f64>()))
        .arg(arg!(--"ips-voter" <NR_PARTIES>)
            .help("Voter process (competitive) on the specified number of parties (i.e., states).")
            .value_parser(value_parser!(usize)))
        .arg(arg!(--"ips-two-si" <BIRTH_AND_DEATH_AND_COMPETE_RATE>)
            .help("Susceptible-infected process with two identical invasive species (states 1 and 2), competing indirectly \
        via the available space, and directly via conversion (i.e., combat).")
            .min_values(3)
            .max_values(3)
            .value_parser(value_parser!(f64))
            .validator(|s| s.parse::<f64>()))
        .arg(arg!(--"ips-sir" <BIRTH_AND_DEATH_RATE>).required(false)
            .help("Susceptible-infected-removed process, specify birth and death rates.")
            .min_values(2)
            .max_values(2)
            .value_parser(value_parser!(f64))
            .validator(|s| s.parse::<f64>()))
        .group(ArgGroup::new("ips-kind")
            .args(&[
                "ips-si",
                "ips-sir",
                "ips-voter",
                "ips-two-si",
                "ips-sir"
            ])
            .required(true))
        // Select initial condition
        .arg(arg!(--"initial-random").required(false)
            .help("Start with random initial condition, where each state has equal probability."))
        .arg(arg!(--"initial-different-particles" <DIFFERENT_AND_PARTICLES>).required(false)
            .help("Start with a list of specified different particles. The other particles \
            will be in the state 0.")
            .min_values(2)
            .value_parser(value_parser!(usize)))
        .group(ArgGroup::new("initial-kind")
            .args(&["initial-random", "initial-different-particles"])
            .required(true))
        // Select halting condition
        .arg(arg!(--"halt-time-passed" <TIME_PASSED>).required(false)
            .help("Stop simulation after a certain specified amount of time as passed.")
            .value_parser(value_parser!(f64))
            .validator(|s| s.parse::<f64>()))
        .arg(arg!(--"halt-steps-recorded" <STEPS>).required(false)
            .help("Stop simulation after a certain specified number of steps have been recorded.")
            .value_parser(value_parser!(u64))
            .validator(|s| s.parse::<u64>()))
        .arg(arg!(--"halt-steps-taken" <STEPS>).required(false)
            .help("Stop simulation after a certain specified number of steps have been taken.")
            .value_parser(value_parser!(u64))
            .validator(|s| s.parse::<u64>()))
        .group(ArgGroup::new("halt-kind")
            .args(&["halt-time-passed", "halt-steps-recorded", "halt-steps-taken"])
            .required(true))
        // Select record condition
        .arg(arg!(--"record-final").required(false)
            .help("Only record the final state."))
        .arg(arg!(--"record-nth-step" <STEP>).required(false)
            .help("Record every nth step.")
            .value_parser(value_parser!(usize)))
        .arg(arg!(--"record-constant-time" <TIME>).required(false)
            .help("Record state at every whole multiple of  the specified time.")
            .value_parser(value_parser!(f64)))
        .group(ArgGroup::new("record-kind")
            .args(&["record-final", "record-nth-step", "record-constant-time"])
            .required(true))
        // Select output kind
        .arg(arg!(--"image-growth").required(false)
            .help("Record output of growth-image type. The output file name must end in .png."))
        .arg(arg!(--"image-gif" <IMG_Y_AND_MS_PER_FRAME>).required(false)
            .help("Record output as a gif. The output file name must end in .gif.")
            .min_values(2)
            .max_values(2)
            .value_parser(value_parser!(u32)))
        .group(ArgGroup::new("image_output_kind")
            .args(&["image-growth", "image-gif"])
            .required(true))
        // Set output file name
        .arg(arg!(--"output" <FILE_NAME>).required(true)
            .help("File output name."))

        .get_matches();

    /* Get the arguments into a pretty form */

    // Make graph from provided arguments
    let graph: Box<dyn Graph>;

    if matches.is_present("graph-grid-nd") {

        let values = matches.get_many::<u64>("graph-grid-nd").unwrap();

        let mut grid_dimensions = vec![];
        let mut grid_glue = vec![];

        for i in values {
            grid_dimensions.push(*i);
            grid_glue.push(true);
        }
        graph = Box::new(
            GridND::from((grid_dimensions, grid_glue))
        )

    } else if matches.is_present("graph-erdos-renyi") {
        let mut values = matches.get_many::<u64>("graph-erdos-renyi").unwrap();

        let nr_points = values.next().unwrap();
        let avg_nr_neighs = values.next().unwrap();

        // The probability that 2 points are connected is related to the avg number of neighbours,
        //  the first of which is noninteger, the second of which might be
        graph = Box::new(
            ErdosRenyi::new(*nr_points, *avg_nr_neighs as f64 / *nr_points as f64, rand::thread_rng())
        )
    } else if matches.is_present("graph-diluted-lattice") {
        let mut values = matches.get_many::<u64>("graph-diluted-lattice").unwrap();

        let dim_x = values.next().unwrap();
        let dim_y = values.next().unwrap();
        let percentile = values.next().unwrap();

        graph = Box::new(
            DilutedLattice::new(*dim_x, *dim_y, *percentile as f64 / 100.0, rand::thread_rng())
        )
    } else {
        panic!("Graph not recognized!");
    }

    println!("Graph:");
    graph.describe(); // Print pretty statistics of the selected graph
    let graph_nr_points = graph.nr_points();

    // Load up default values for ips rules
    let ips_rules: Box<dyn IPSRules>;
    let coloration: Box<dyn Coloration>;

    // Make ips from provided arguments
    if matches.is_present("ips-si") {
        let mut values = matches.get_many::<f64>("ips-si").unwrap();
        assert_eq!(values.len(), 2); // raise argument error
        let birth_rate = *values.next().unwrap();
        let death_rate = *values.next().unwrap();

        coloration = Box::new(SIProcess {
            birth_rate,
            death_rate,
        });

        ips_rules = Box::new(SIProcess {
            birth_rate,
            death_rate,
        });
    } else if matches.is_present("ips-voter") {
        let nr_parties = *matches.get_one::<usize>("ips-voter").unwrap();

        coloration = Box::new(VoterProcess {
            nr_parties,
            change_rate: 1.0, // With this setup, we can't have two parameters of different types
            // in the same process; nr_parties being a usize excludes the possibility to parameterize
            // change_rate (a f64)
        });

        ips_rules = Box::new(VoterProcess {
            nr_parties,
            change_rate: 1.0,
        });
    } else if matches.is_present("ips-two-si") {
        let mut values = matches.get_many::<f64>("ips-two-si").unwrap();
        assert_eq!(values.len(), 3); // raise argument error
        let birth_rate = *values.next().unwrap();
        let death_rate = *values.next().unwrap();
        let compete_rate = *values.next().unwrap();

        coloration = Box::new(TwoSIProcess {
            birth_rate,
            death_rate,
            compete_rate,
        });

        ips_rules = Box::new(TwoSIProcess {
            birth_rate,
            death_rate,
            compete_rate,
        });
    } else if matches.is_present("ips-sir") {
        let mut values = matches.get_many::<f64>("ips-sir").unwrap();
        assert_eq!(values.len(), 2); // raise argument error
        let birth_rate = *values.next().unwrap();
        let death_rate = *values.next().unwrap();

        coloration = Box::new(SIRProcess {
            birth_rate,
            death_rate,
        });

        ips_rules = Box::new(SIRProcess {
            birth_rate,
            death_rate,
        });
    } else {
        panic!("No other processes implemented")
    }

    println!("Interacting particle system:");
    ips_rules.describe();
    println!();

    // Load up default initial condition
    let initial_condition: Vec<usize>;

    // Make initial condition from provided arguments
    if matches.is_present("initial-random") {
        initial_condition = assemble_random_initial_condition(ips_rules.all_states(), graph_nr_points)
    } else if matches.is_present("initial-different-particles") {
        let mut values = matches.get_many::<usize>("initial-different-particles").unwrap();
        let different_state = *values.next().unwrap();
        let different_particles: HashSet<&usize> = values.collect();
        let mut different_particles_hashmap: HashMap<u64, usize> = HashMap::new();

        for i in different_particles {
            different_particles_hashmap.insert(*i as u64, different_state);
        }

        initial_condition = assemble_initial_condition(0, different_particles_hashmap, graph.nr_points())
    } else {
        panic!("Initial condition not recognized!")
    }

    // Load up default halting condition
    let halting_condition: HaltCondition;

    // Make halting condition from provided arguments
    if matches.is_present("halt-time-passed") {
        halting_condition = HaltCondition::TimePassed(
            *matches.get_one::<f64>("halt-time-passed").unwrap()
        )
    } else if matches.is_present("halt-steps-recorded") {
        halting_condition = HaltCondition::StepsRecorded(
            *matches.get_one::<u64>("halt-steps-recorded").unwrap()
        )
    } else if matches.is_present("halt-steps-taken") {
        halting_condition = HaltCondition::StepsTaken(
            *matches.get_one::<u64>("halt-steps-taken").unwrap()
        )
    } else {
        panic!("Halting condition not recognized!")
    }

    // Load up default record condition
    let mut record_condition = RecordCondition::Final();

    // Make initial condition from provided arguments
    if matches.is_present("record-final") {
        record_condition = RecordCondition::Final()
    } else if matches.is_present("record-nth-step") {
        record_condition = RecordCondition::EveryNthStep(
            *matches.get_one::<usize>("record-nth-step").unwrap()
        )
    } else if matches.is_present("record-constant-time") {
        record_condition = RecordCondition::ConstantTime(
            *matches.get_one::<f64>("record-constant-time").unwrap()
        )
    }


    /* Run simulation */
    let now = Instant::now();

    let (solution, final_state, time_simulated, steps_recorded, steps_taken)
        = particle_system_solver(
        ips_rules,
        graph,
        initial_condition,
        halting_condition,
        record_condition,
        rand::thread_rng(),
    );

    let elapsed = now.elapsed();

    /* Give some statistics of the simulation */
    println!("Thought for {:.2?}.", elapsed);
    println!("Simulated {:.2?} time units, in which {} steps were taken, and {} were recorded.",
             time_simulated, steps_taken, steps_recorded);
    let mut state_counts: HashMap<usize, usize> = HashMap::new();
    for particle_state in final_state {
        state_counts.insert(particle_state, state_counts.get(&particle_state).unwrap_or(&0usize) + 1);
    }

    /* Give some statistics of the final state */
    println!("The final state has the following counts: {:?}.", state_counts);

    /* Pack simulation into image */
    if matches.is_present("image-growth") {
        let img_x = graph_nr_points;
        let img_name = matches.get_one::<String>("output").unwrap();
        assert_eq!(img_name[img_name.len() - 4..], *".png");

        save_as_growth_img(
            coloration,
            solution,
            img_name,
            img_x as u32,
        )
    } else if matches.is_present("image-gif") {
        let mut values = matches.get_many::<u32>("image-gif").unwrap();
        let img_y = values.next().unwrap();
        let img_x = graph_nr_points as u32 / img_y;
        let ms_per_frame = values.next().unwrap();
        let img_name = matches.get_one::<String>("output").unwrap();
        assert_eq!(img_name[img_name.len() - 4..], *".gif");

        save_as_gif(
            coloration,
            solution,
            img_name,
            img_x,
            *img_y,
            *ms_per_frame,
        )
    } else {
        panic!("Image output kind not recognized!");
    }

    /* Done */
}