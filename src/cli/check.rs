//! Functions and types for the `check` command.

use clap::Args;
// We need to use clap::Args to parse the arguments for each command
#[derive(Debug, Args)]
pub struct Check;
// pub to let parent "cli.rs" access structs.
