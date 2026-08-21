// `main.rs` is the binary entry-point, e.g. for CLIs

use clap::Parser;

mod cli;

/// From data requests to reproducible subsets.
#[derive(Debug, Parser)] // clap::Parser for the whole CLI
#[command(version, about, arg_required_else_help = true)]
struct Cli {
  #[command(subcommand)]
    command: cli::Commands,
    // Connect Commands enum to Cli struct in `cli.rs`.
}

fn main() {
    let cli = Cli::parse(); // look for supported commands

    match cli.command {
        cli::Commands::Build(_) => todo!(), // if Build was chosen, todo...
        cli::Commands::Check(_) => todo!(),
        cli::Commands::CreateRequest(_) => todo!(),
        cli::Commands::Subset(_) => todo!(),
    }
    // panics if used - will be implemented later.
}
