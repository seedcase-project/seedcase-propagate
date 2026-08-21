use clap::Subcommand; // parses command-line subcommands into the user-defined enum.

pub mod build;
pub mod check;
pub mod create_request;
pub mod subset;

#[derive(Debug, Subcommand)] // the enum of commands
pub enum Commands {
    Build(build::Build), // Commands can be a Build variant, and that variant contains a build::Build value.
    Check(check::Check),
    CreateRequest(create_request::CreateRequest),
    Subset(subset::Subset),
}
