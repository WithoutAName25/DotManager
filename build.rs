use clap::CommandFactory;
use clap_complete::Shell::{Bash, Fish, Zsh};

include!("src/args.rs");

fn main() -> std::io::Result<()> {
    let mut cmd = <Args as CommandFactory>::command();

    let generated_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("target/generated");
    
    let completions_dir = generated_path.join("completions");
    std::fs::create_dir_all(&completions_dir)?;
    
    clap_complete::generate_to(Bash, &mut cmd, "dot-manager", &completions_dir)?;
    clap_complete::generate_to(Fish, &mut cmd, "dot-manager", &completions_dir)?;
    clap_complete::generate_to(Zsh, &mut cmd, "dot-manager", &completions_dir)?;
    
    let manpage_dir = generated_path.join("manpage");
    std::fs::create_dir_all(&manpage_dir)?;
    
    clap_mangen::generate_to(cmd, manpage_dir)?;
    
    Ok(())
}