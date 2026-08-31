use clap::{Args, Subcommand};
use std::path::PathBuf;
// Subcommand: parses command-line subcommands into the user-defined enum.
// Args: We need to use clap::Args to parse the arguments for each command

pub mod build;
pub mod check;
pub mod create_request;
pub mod subset;

#[derive(Debug, Args)]
pub struct BuildArgs {
    /// Path to the source directory.
    #[arg(long, default_value = "datapackage.json")]
    pub source: PathBuf,

    /// Directory where the built output should be written.
    #[arg(long, default_value = "docs/requests/")]
    pub output_dir: PathBuf,
}

#[derive(Debug, Args)]
pub struct CheckArgs {
    /// Path to the instructions request.yaml file.
    #[arg(long)]
    pub request: PathBuf,

    /// Path to the datapackage.json file.
    #[arg(short, long, default_value = "datapackage.json")]
    pub source: PathBuf,
}

#[derive(Debug, Args)]
pub struct CreateRequestArgs {
    /// Path to the datapackage.json file.
    #[arg(short, long, default_value = "datapackage.json")]
    pub source: PathBuf,
}

#[derive(Debug, Args)]
pub struct SubsetArgs {
    /// Path to the instructions request.yaml file.
    #[arg()]
    pub request: PathBuf,

    /// Path to the datapackage.json file.
    #[arg(short, long, default_value = "datapackage.json")]
    pub source: PathBuf,

    /// Directory where the subsetted output should be written.
    #[arg(short, long, default_value = "subset/")]
    pub output_dir: PathBuf,

    /// Show the subsetting plan without executing it.
    #[arg(short = 'p', long, default_value = "false")]
    pub show_plan: bool,
}

#[derive(Debug, Subcommand)] // the enum of commands
pub enum Commands {
    /// Build and set up the request web app for including in static websites.
    Build(BuildArgs),
    /// Validate the compatibility between request.yaml and datapackage.json.
    Check(CheckArgs),
    /// Interactively create a request.yaml file from source metadata.
    CreateRequest(CreateRequestArgs),
    /// Produce a subset based on the request.yaml and your metadata.
    Subset(SubsetArgs),
}
