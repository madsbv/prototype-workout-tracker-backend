#![warn(clippy::all, clippy::pedantic)]
// #![warn(clippy::cargo)]
#![allow(
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::struct_excessive_bools
)]

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
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
fn main() {
    let args = Cli::parse();
}
