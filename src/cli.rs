use clap::{Args, Subcommand};
use std::path::PathBuf;
// use crate::core::metadata::Package;
// use crate::core::request::Request;
// use crate::core::Rap;
// use crate::core::check::{check_request;

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

/// Function used in the CLI to check the request file against the package metadata.
///
/// # Arguments:
///
/// - `args`: This is the [`CheckArgs`] struct that contains the arguments for
///   the CLI command.
///
/// # Errors
///
/// Prints any failed checks to `stderr`.
#[allow(unused, clippy::needless_pass_by_value)]
pub fn check(args: CheckArgs) {
    // TODO: Make use of loggers to print information when e.g. using verbose?

    // TODO: Not sure whether we should do custom `match` for errors, e.g. via `eprintln`?
    // let package_metadata: Package = read_package_metadata(args.source)?;
    // let request: Request = read_request(args.request)?;

    // let rap = Rap {
    //   package: package_metadata,
    //   request: request
    // };

    // let checked_request = check_request(&Rap);

    // TODO: Look into better/prettier output, via clap Styles?
    // match checked_request {
    //   Ok(output) => println!("Request checks passed!"),
    //   Err(e) => eprint!("Request checks failed: {:?}", e)
    // }

    todo!("Planned")
}
