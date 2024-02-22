use anyhow::Result;

use crate::args::{Args, Command};
use crate::commands::add::run_add;
use crate::commands::setup::run_setup;

mod setup;
mod add;

pub fn run(args: Args) -> Result<()> {
    match args.command {
        Command::Setup => run_setup(),
        Command::Add { path } => run_add(path),
    }
}