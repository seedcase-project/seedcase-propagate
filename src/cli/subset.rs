//! Functions and types for the `subset` command.

use clap::Args;
// We need to use clap::Args to parse the arguments for each command
#[derive(Debug, Args)] // Args parses this specific commands arguments
pub struct Subset;
// pub to let parent "cli.rs" access structs.
