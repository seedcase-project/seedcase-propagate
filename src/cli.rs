use clap::{Subcommand, Args};
// Subcommand: parses command-line subcommands into the user-defined enum.
// Args: We need to use clap::Args to parse the arguments for each command

pub mod build;
pub mod check;
pub mod create_request;
pub mod subset;

#[derive(Debug, Args)]
pub struct Build;

#[derive(Debug, Args)]
pub struct Check;

#[derive(Debug, Args)]
pub struct CreateRequest;

#[derive(Debug, Args)]
pub struct Subset;

#[derive(Debug, Subcommand)] // the enum of commands
pub enum Commands {
    Build(Build), // Commands can be a Build variant, and that variant contains a Build value.
    Check(Check),
    CreateRequest(CreateRequest),
    Subset(Subset),
}
