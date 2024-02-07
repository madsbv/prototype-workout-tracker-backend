// PURPOSE: CLI interface to the main functionality of lib.rs.
// TODO: Figure out how to use Clap. https://docs.rs/clap/latest/clap/
// TODO: TUI library? Or text entry loop...

use clap::Parser;

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

#[derive(Subcommand)]
enum Command {
    print_exercise { name: String },
    print_exercise_history { name: String },
}
fn main() {
    let args = Args::parse();
}
