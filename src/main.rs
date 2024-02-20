#![warn(clippy::all, clippy::pedantic)]
// #![warn(clippy::cargo)]
#![allow(
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::struct_excessive_bools
)]

// PURPOSE: CLI interface to the main functionality of lib.rs.
// TODO: Figure out how to use Clap. https://docs.rs/clap/latest/clap/

use clap::{Parser, Subcommand};
use std::{fs, io};
use workout_tracker_backend::em_exercise_data::parse_em_spec_csv_to_exercises;
use workout_tracker_backend::fuzzy_exercise_matching::search_exercises;
use workout_tracker_backend::strong_data::StrongData;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // TODO: Define some basic commands, figure out how to use
    // QUESTION: Can I make some variables Option types?
    // A: Yes: https://docs.rs/clap/latest/clap/_derive/_tutorial/chapter_0/index.html
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
// Allow non-rusty names to match CLI conventions more closely
#[allow(non_camel_case_types)]
enum Command {
    print_exercise { name: String },
    print_exercise_history { name: String },
    // TODO: Is there a file specification type?
    import_strong_data { file: String },
}
// fn main() {
//     let args = Args::parse();
// }

fn run() -> anyhow::Result<()> {
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let _ = run();
    if let Err(err) = run() {
        println!("{}", err);
    }
    Ok(())
}
