use std::io;
use std::io::Write;

use anyhow::{Error, Result};

use crate::config::{Config, get_config_dot_path, read_config, write_config};
use crate::path_utils::DotPath;

pub fn run_setup() -> Result<()> {
    let mut current = Config::default();
    let config_path = get_config_dot_path()?;
    if let Ok(config) = read_config(&config_path.path) {
        current = config;
    } else if let Ok(synced_config) = read_config(&config_path.path_in_synced_folder) {
        current = synced_config;
    }

    query_synced_folder(&mut current)?;
    query_files_to_prioritize(&mut current)?;

    let sync_config = query_sync_config(&config_path)?;

    write_config(if sync_config { &config_path.path_in_synced_folder } else { &config_path.path }, &current)?;
    if sync_config {
        std::fs::create_dir_all(config_path.path.parent().ok_or(Error::msg("Could not create parent directories"))?)?;
        std::fs::remove_file(&config_path.path).unwrap_or(());
        match std::os::unix::fs::symlink(&config_path.path_in_synced_folder, &config_path.path) {
            Ok(_) => {
                println!("Create Symlink from {} to config in {}", config_path.path, current.synced_folder)
            }
            Err(e) => {
                eprintln!("Failed to create Symlink: {}", e)
            }
        }
    }

    Ok(())
}

fn query_input() -> Result<String> {
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

fn query_synced_folder(current: &mut Config) -> Result<()> {
    print!("Folder where synced config files should be stored [{}]: ", current.synced_folder);

    let input = query_input()?;

    let synced_folder_input = input.trim();
    if synced_folder_input != "" {
        current.synced_folder = synced_folder_input.to_string();
    }
    Ok(())
}

fn query_files_to_prioritize(current: &mut Config) -> Result<()> {
    print!("Which file should be used when both already exists?\n\
    The other file is copied to filename.old.\n\
    (S)ynced or (L)ocal [Default: {}]: ",
           if current.conflicts.prioritize_synced_files { "Synced" } else { "Local" });

    let input = query_input()?;

    match input.trim().to_lowercase().as_str() {
        "" => { Ok(()) }
        "s" => {
            current.conflicts.prioritize_synced_files = true;
            Ok(())
        }
        "l" => {
            current.conflicts.prioritize_synced_files = false;
            Ok(())
        }
        _ => {
            query_files_to_prioritize(current)
        }
    }
}

fn query_sync_config(config_path: &DotPath) -> Result<bool> {
    print!("Should the config file from DotManager ({}) also be synced? [Y/n]", config_path.relative_to_home);

    let input = query_input()?;

    match input.trim().to_lowercase().as_str() {
        "" | "y" => { Ok(true) }
        "n" => { Ok(false) }
        _ => { query_sync_config(config_path) }
    }
}
