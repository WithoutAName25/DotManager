use std::env::current_dir;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::{create_dir, create_dir_all, rename};
use std::os::unix::fs::symlink;
use std::str::FromStr;

use anyhow::Result;
use camino::Utf8PathBuf;
use clap::{arg, Parser};

/// CLI utility for managing syncing of dotfiles
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Synced dotfiles location
    #[arg(short, long, default_value = "~/.dotfiles")]
    synced_folder: String,

    /// Path to file or folder that should be synced
    path: String,
}

struct DotPath {
    relative_to_home: String,
    path: Utf8PathBuf,
    path_in_synced_folder: Utf8PathBuf,
}

struct Config {
    home_dir: Utf8PathBuf,
    synced_folder: Utf8PathBuf,
    path: DotPath,
}

#[derive(Debug)]
struct ProcessArgsError {
    pub message: String,
}

impl Display for ProcessArgsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid Arguments: {}", self.message)
    }
}

impl Error for ProcessArgsError {}

fn args_error(message: &str) -> ProcessArgsError {
    ProcessArgsError {
        message: message.to_string()
    }
}

fn get_home_dir() -> Option<Utf8PathBuf> {
    Some(Utf8PathBuf::from_path_buf(dirs::home_dir()?).ok()?.canonicalize_utf8().ok()?)
}

fn get_absolute_path(path: String, home_dir: &Utf8PathBuf) -> Result<Utf8PathBuf, ProcessArgsError> {
    let path = path.strip_suffix("/").unwrap_or(path.as_str());
    if let Some(path_stripped) = path.strip_prefix("~/") {
        Ok(home_dir.join(path_stripped))
    } else if path.starts_with("/") {
        Ok(Utf8PathBuf::from_str(&*path).map_err(|_| args_error("Failed to parse path"))?)
    } else {
        Ok(Utf8PathBuf::from_path_buf(
            current_dir()
                .map_err(|_| args_error("Couldn't get current directory to resolve relative path!"))?
        )
            .map_err(|_| args_error("Failed to parse current directory!"))?
            .join(path))
    }
}

fn get_dot_path(
    path: String,
    home_dir: &Utf8PathBuf,
    synced_folder: &Utf8PathBuf
) -> Result<DotPath, ProcessArgsError> {
    let path = get_absolute_path(path, home_dir)?;
    let relative_to_home = path
        .strip_prefix(home_dir)
        .map_err(|_| args_error("failed to determine relative to home dir path"))?;
    let path_in_synced_folder = synced_folder.join(relative_to_home);
    return Ok(DotPath {
        relative_to_home: relative_to_home.as_str().to_string(),
        path,
        path_in_synced_folder,
    });
}

fn process_args(args: Args) -> Result<Config, ProcessArgsError> {
    let home_dir = get_home_dir().ok_or(args_error("Couldn't find home dir!"))?;
    let synced_folder = get_absolute_path(args.synced_folder, &home_dir)?;
    let path = get_dot_path(args.path, &home_dir, &synced_folder)?;

    return Ok(Config { home_dir, synced_folder, path });
}

fn dot_manager(config: Config) {
    if config.path.path_in_synced_folder.exists() {
        eprintln!("Unimplemented: {} already exists!", config.path.path_in_synced_folder);
        return;
    } else if config.path.path.exists() {
        if let Some(parent) = config.path.path_in_synced_folder.parent() {
            if let Err(_) = create_dir_all(parent) {
                eprintln!("Could not create parent dirs in synced folder!");
                return;
            }
        }
        if let Err(_) = rename(&config.path.path, &config.path.path_in_synced_folder) {
            eprintln!(
                "Could not move config file(s) from {} to {}",
                config.path.path,
                config.path.path_in_synced_folder);
            return;
        }
    } else if config.path.path_in_synced_folder.ends_with("/") {
        if let Err(_) = create_dir(&config.path.path_in_synced_folder) {
            eprintln!("Failed to create directory {}", config.path.path_in_synced_folder);
            return;
        }
    }

    match symlink(&config.path.path_in_synced_folder, &config.path.path) {
        Ok(_) => {
            println!("Create Symlink from {} to config in {}", config.path.path, config.synced_folder)
        }
        Err(e) => {
            eprintln!("Failed to create Symlink: {}", e)
        }
    }
}

fn main() {
    match process_args(Args::parse()) {
        Ok(config) => {
            dot_manager(config)
        }
        Err(err) => {
            eprintln!("{}", err)
        }
    }
}
