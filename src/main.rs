// `main.rs` is the binary entry-point, e.g. for CLIs

use clap::Parser;

mod cli;

/// From data requests to reproducible subsets.
#[derive(Debug, Parser)]
#[command(version, about, arg_required_else_help = true)]
struct Cli {}

fn main() {
    Cli::parse();
}
