use clap::{Parser, Subcommand};
#[cfg(feature = "completions")]
use clap_complete::Shell;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Subcommand, Debug)]
#[command()]
pub enum Command {
    /// Add a file or folder to be synced
    #[command()]
    Add {
        /// The file or folder path to be synced
        path: String,
    },
    /// Generate shell completions
    #[cfg(feature = "completions")]
    #[command()]
    Completions {
        /// The shell to generate completions for
        shell: Shell,
    },
    /// Configure dot-manager
    #[command()]
    Setup,
}
