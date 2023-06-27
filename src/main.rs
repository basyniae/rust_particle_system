use std::collections::{HashMap, HashSet};
use std::time::Instant;
use clap::{arg, ArgGroup, command, value_parser};
use crate::solver::assemble_initial_condition::{assemble_initial_condition, assemble_random_initial_condition};
use crate::solver::graph::{Graph};
use crate::solver::ips_rules::{IPSRules};
use crate::solver::{HaltCondition, particle_system_solver, RecordCondition};
use crate::solver::graph::grid_n_d::GridND;
use crate::solver::ips_rules::contact_process::ContactProcess;
use crate::visualization::{Coloration, save_as_gif, save_as_growth_img};

pub mod visualization;
pub mod solver;

fn main() {
    let matches = command!("cmd")
        // Select graph
        .arg(arg!(--"graph-grid-nd" <DIMENSIONS>).required(false)
            .help("n-dimensional grid. Specify dimensions.")
            .min_values(1)
            .multiple_values(true)
            .value_parser(value_parser!(u64))
            .validator(|s| s.parse::<u64>()))
        .arg(arg!(--"graph-erdos-renyi" <DIMENSIONS_AND_COUNT>).required(false)
            .help("Erdos-Renyi graph. Specify dimensions and average nr of neighbours per particle.")
            .min_values(2)
            .max_values(2)
            .multiple_values(true))
        .group(ArgGroup::new("graph-kind")
            .args(&["graph-grid-nd", "graph-erdos-renyi"])
            .required(true)
        )
        // Select IPS
        .arg(arg!(--"ips-contact-process" <BIRTH_AND_DEATH_RATE>).required(false)
            .help("Contact process, specify birth and death rates.")
            .min_values(2)
            .max_values(2)
            .value_parser(value_parser!(f64))
            .validator(|s| s.parse::<f64>()))
        .arg(arg!(--"ips-sir-process" <SIR_PARAMS>)
            .help("unimplemented"))
        .group(ArgGroup::new("ips-kind")
            .args(&["ips-contact-process", "ips-sir-process"])
            .required(true))
        // Select initial condition
        .arg(arg!(--"initial-random").required(false)
            .help("Start with random initial condition"))
        .arg(arg!(--"initial-different-particles" <DIFFERENT_AND_PARTICLES>).required(false)
            .help("Start with a list of specified different particles. The other particles will be in the state 0.")
            .min_values(2)
            .value_parser(value_parser!(usize)))
        .group(ArgGroup::new("initial-kind")
            .args(&["initial-random", "initial-different-particles"])
            .required(true))
        // Select halting condition
        .arg(arg!(--"halt-time-passed" <TIME_PASSED>).required(false)
            .value_parser(value_parser!(f64))
            .validator(|s| s.parse::<f64>()))
        .arg(arg!(--"halt-steps-recorded" <STEPS>).required(false)
            .value_parser(value_parser!(u64))
            .validator(|s| s.parse::<u64>()))
        .arg(arg!(--"halt-steps-taken" <STEPS>).required(false)
            .value_parser(value_parser!(u64))
            .validator(|s| s.parse::<u64>()))
        .group(ArgGroup::new("halt-kind")
            .args(&["halt-time-passed", "halt-steps-recorded", "halt-steps-taken"])
            .required(true))
        // Select record condition
        .arg(arg!(--"record-final").required(false))
        .arg(arg!(--"record-nth-step" <STEP>).required(false)
            .value_parser(value_parser!(usize)))
        .arg(arg!(--"record-constant-time" <TIME>).required(false)
            .value_parser(value_parser!(f64)))
        .group(ArgGroup::new("record-kind")
            .args(&["record-final", "record-nth-step", "record-constant-time"])
            .required(true))
        // Select output kind
        .arg(arg!(--"image-growth").required(false))
        .arg(arg!(--"image-gif" <IMG_Y_AND_MS_PER_FRAME>).required(false)
            .min_values(2)
            .max_values(2)
            .value_parser(value_parser!(u32)))
        .group(ArgGroup::new("image_output_kind")
            .args(&["image-growth", "image-gif"])
            .required(true))
        // Set output file name
        .arg(arg!(--"output" <FILE_NAME>).required(true))

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
        // graph_kind = GraphKind::ErdosRenyi;
        panic!("Erdos-Renyi graph not implemented")
    } else {
        panic!("Graph not recognized!");
    }

    let graph_nr_points = graph.nr_points();

    // Load up default values for ips rules
    let ips_rules: Box<dyn IPSRules>;
    let coloration: Box<dyn Coloration>;

    // Make ips from provided arguments
    if matches.is_present("ips-contact-process") {
        let mut contact_process_parameters: Vec<f64> = vec![];
        let mut values = matches.get_many::<f64>("ips-contact-process").unwrap();
        assert_eq!(values.len(), 2); // raise argument error
        contact_process_parameters.push(*values.next().unwrap()); // birth rate
        contact_process_parameters.push(*values.next().unwrap()); // death rate

        coloration = Box::new(ContactProcess {
            birth_rate: *contact_process_parameters.get(0).unwrap(),
            death_rate: *contact_process_parameters.get(1).unwrap(),
        });

        ips_rules = Box::new(ContactProcess {
            birth_rate: *contact_process_parameters.get(0).unwrap(),
            death_rate: *contact_process_parameters.get(1).unwrap(),
        })
    } else {
        panic!("No other processes implemented")
    }

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

    let solution = particle_system_solver(
        ips_rules,
        graph,
        initial_condition,
        halting_condition,
        record_condition,
        rand::thread_rng(),
    );

    let elapsed = now.elapsed();
    println!("Simulation time: {:.2?}", elapsed);

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