// use crate::core::{Rap, CheckedRap};
// use crate::core::check::check_request;
// use crate::core::metadata::{Package, read_package_metadata};
// use crate::core::subset::{create_plan, SubsettedResource, subset_resources,
// write_resources}; use std::path::PathBuf; // Or `Path`?

#[allow(unused)]
pub struct SubsetArgs;

/// Creates the requested subsets from the data package.
///
/// # Argument:
///
/// - `args`: This is the [`SubsetArgs`] struct that contains the CLI arguments
///   for the `subset` command.
#[allow(unused, clippy::needless_pass_by_value)]
pub fn subset(args: SubsetArgs) {
    // TODO: Will we need to include some failure match here for reading issues?
    // let request: Request = read_request(args.request)?;

    // if args.show_plan {
    //   // TODO: Not sure println is right approach.
    //   println!("Plan for subsetting: {:?}", create_plan(args.request));
    //   // End early.
    //   return;
    // };

    // TODO: Will we need to include some failure match here for reading issues?
    // let package_metadata: Package = read_package_metadata(args.source)?;

    // let rap: CheckedRap = check_request(Rap {
    //   package: package_metadata,
    //   request: request
    // });

    // TODO: Not sure if Vec or Arc should be used.
    // TODO: Need to include failure matches here?
    // let subsetted_resources: Arc<[SubsettedResource]> = subset_resources(rap)?;

    // TODO: Not sure if Path or PathBuf is better here.
    // let written_resources: Result<Path, Error> =
    // write_resources(subsetted_resources, args.output_dir);

    // TODO: Need to include failure matches here?

    // match written_resources {
    //   Ok(paths) => println!("Created subset: {:?}", paths),
    //   Err(e) => eprintln!("Failed to write ")
    // }

    todo!("Planned")
}

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
    // TODO: Switch to using enum for e.g. http, gh, and file
    /// Source to the metadata file.
    #[arg(long, default_value = "datapackage.json")]
    pub source: PathBuf,

    /// Directory where the built output should be written.
    #[arg(long, default_value = "docs/requests/")]
    pub output_dir: PathBuf,
}

#[derive(Debug, Args)]
pub struct CheckArgs {
    /// Path to the request file.
    #[arg(long)]
    pub request: PathBuf,

    // TODO: Switch to enum once its made
    /// Source to the metadata file.
    #[arg(short, long, default_value = "datapackage.json")]
    pub source: PathBuf,
}

#[derive(Debug, Args)]
pub struct CreateRequestArgs {
    /// Source to the metadata file.
    #[arg(short, long, default_value = "datapackage.json")]
    pub source: PathBuf,
}

#[derive(Debug, Args)]
pub struct SubsetArgs {
    /// Path to the request file.
    #[arg(long)]
    pub request: PathBuf,

    /// Source to the metadata file.
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
    /// Check that the request file contains the correct metadata from the data package.
    Check(CheckArgs),
    /// Interactively create a request file from source metadata.
    CreateRequest(CreateRequestArgs),
    /// Produce a subset based on the request and the metadata file of the data package.
    Subset(SubsetArgs),
}
