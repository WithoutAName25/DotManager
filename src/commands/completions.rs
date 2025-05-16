use crate::args::Args;
use anyhow::Result;
use clap::CommandFactory;
use clap_complete::{generate, Shell};

pub fn run_completions(shell: Shell) -> Result<()> {
    let mut cmd = Args::command();
    
    generate(shell, &mut cmd, "dot-manager", &mut std::io::stdout());
    
    Ok(())
}