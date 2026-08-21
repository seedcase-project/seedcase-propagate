//! Functions and types for the `build` command.

use clap::Args;
// We need to use clap::Args to parse the arguments for each command
#[derive(Debug, Args)]
pub struct Build;
// pub to let parent "cli.rs" access structs.
