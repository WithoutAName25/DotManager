use anyhow::Result;

use crate::args::{Args, Command};
use crate::commands::add::run_add;
#[cfg(feature = "completions")]
use crate::commands::completions::run_completions;
use crate::commands::setup::run_setup;

mod add;
#[cfg(feature = "completions")]
mod completions;
mod setup;

pub fn run(args: Args) -> Result<()> {
    match args.command {
        Command::Setup => run_setup(),
        Command::Add { path } => run_add(path),
        #[cfg(feature = "completions")]
        Command::Completions { shell } => run_completions(shell),
    }
}
