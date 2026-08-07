// `main.rs` is the binary entry-point, e.g. for CLIs

use clap::Parser;

mod cli;

/// Submit requests for data in a data package.
#[derive(Debug, Parser)]
#[command(version, about, arg_required_else_help = true)]
struct Cli {}

fn main() {
    Cli::parse();
}
