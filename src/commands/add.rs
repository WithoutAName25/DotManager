use std::fs;
use std::fs::rename;
use std::os::unix::fs::symlink;

use anyhow::{Error, Result};
use camino::Utf8Path;

use crate::config::load_config;
use crate::path_utils::get_dot_path;

pub fn run_add(path: String) -> Result<()> {
    let config = load_config()?;
    let dot_path = get_dot_path(&config, &path)?;

    let path_exists = dot_path.path.try_exists().unwrap_or(false);
    if path_exists
        && fs::symlink_metadata(&dot_path.path)?
            .file_type()
            .is_symlink()
    {
        return Err(Error::msg(
            dot_path.path.to_string() + " is already synced!",
        ));
    }

    let synced_path_exists = dot_path.path_in_synced_folder.try_exists().unwrap_or(false);
    match (path_exists, synced_path_exists) {
        (false, _) => {
            if let Some(parent) = dot_path.path.parent() {
                fs::create_dir_all(parent)?;
            }
        }
        (true, false) => {
            if let Some(parent) = dot_path.path_in_synced_folder.parent() {
                fs::create_dir_all(parent)?;
            }
            rename(&dot_path.path, &dot_path.path_in_synced_folder)?
        }
        (true, true) => {
            if config.conflicts.prioritize_synced_files {
                let local_backup = dot_path.path.as_str().to_owned() + ".old";
                let local_backup_path = Utf8Path::new(&local_backup);
                if local_backup_path.try_exists().unwrap_or(false) {
                    return Err(Error::msg(local_backup + " already exists!"));
                }
                rename(&dot_path.path, local_backup_path)?;
                println!("The original local file was moved to {}", local_backup);
            } else {
                let synced_backup = dot_path.path_in_synced_folder.as_str().to_owned() + ".old";
                let synced_backup_path = Utf8Path::new(&synced_backup);
                if synced_backup_path.try_exists().unwrap_or(false) {
                    return Err(Error::msg(synced_backup + " already exists!"));
                }
                rename(&dot_path.path_in_synced_folder, synced_backup_path)?;
                println!("The original synced file was moved to {}", synced_backup);
                rename(&dot_path.path, &dot_path.path_in_synced_folder)?;
            }
        }
    }

    symlink(&dot_path.path_in_synced_folder, &dot_path.path)
        .map_err(|e| Error::msg("Failed to create Symlink: ".to_owned() + &*e.to_string()))?;
    println!(
        "Create Symlink from {} to config in {}",
        dot_path.path, config.synced_folder
    );
    Ok(())
}

