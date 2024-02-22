use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Subcommand, Debug)]
#[command()]
pub enum Command {
    #[command()]
    Setup,
    Add {
        /// The file or folder path to be synced
        path: String,
    },
}
