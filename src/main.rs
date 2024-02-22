use clap::Parser;
use args::Args;

use crate::commands::run;

mod config;
mod commands;
mod path_utils;
mod args;

fn main() {
    match run(Args::parse()) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{}", err)
        }
    }
}
