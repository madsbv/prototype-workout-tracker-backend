// TODO: Tighten up linting once csv parsing has been solved.
#![allow(dead_code)]
#![allow(unused_imports)]

// PURPOSE: CLI interface to the main functionality of lib.rs.
// TODO: Figure out how to use Clap. https://docs.rs/clap/latest/clap/

use clap::{Parser, Subcommand};
use std::io;
use workout_tracker::StrongData;

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
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        let record: StrongData = result?;
        println!("{:#?}", record);
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let _ = run();
    if let Err(err) = run() {
        println!("{}", err);
    }
    Ok(())
}
