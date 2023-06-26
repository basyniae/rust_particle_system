use std::{env, io};
use std::io::Error;
use std::time::Instant;
use clap::Parser;
use crate::solver::ips_rules::IPSRules;
use crate::solver::particle_system_solver;
use crate::test_scripts::{contact_process_gif, contact_process_img, contact_process_img_random_initial, graph_tester, sir_process_gif, sir_process_img, sir_process_img_random_initial, three_voter_process_img, voter_process_gif, voter_process_img};
use crate::visualization::save_image;

pub mod visualization;
pub mod solver;
pub mod test_scripts;

/// Search for a pattern in a file and display the lines that contain it
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf
}

fn main() {
    // let args = Args::parse();

    contact_process_img();
}