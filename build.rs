use clap::CommandFactory;
use clap_complete::generate_to;
use clap_complete::Shell::{Bash, Fish, Zsh};

include!("src/args.rs");

fn main() -> std::io::Result<()> {
    let mut cmd = <Args as CommandFactory>::command();

    let generated_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("target/generated");
    
    let completions_dir = generated_path.join("completions");
    std::fs::create_dir_all(&completions_dir)?;
    
    generate_to(Bash, &mut cmd, "dot-manager", &completions_dir)?;
    generate_to(Fish, &mut cmd, "dot-manager", &completions_dir)?;
    generate_to(Zsh, &mut cmd, "dot-manager", &completions_dir)?;
    
    let manpage_dir = generated_path.join("manpage");
    std::fs::create_dir_all(&manpage_dir)?;
    
    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;
    std::fs::write(manpage_dir.join("dot-manager.1"), buffer)?;
    
    Ok(())
}