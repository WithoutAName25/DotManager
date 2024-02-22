use anyhow::{Error, Result};
use camino::Utf8Path;
use serde::{Deserialize, Serialize};
use crate::path_utils::{DotPath, get_dot_path};

const CONFIG_PATH: &str = "~/.config/dot-manager/config.toml";

pub fn get_config_dot_path() -> Result<DotPath> {
    get_dot_path(&Config::default(), &CONFIG_PATH.to_string())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub synced_folder: String,
    pub conflicts: ConflictResolveStrategy,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            synced_folder: "~/.dotfiles".to_string(),
            conflicts: ConflictResolveStrategy {
                prioritize_synced_files: true
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConflictResolveStrategy {
    pub prioritize_synced_files: bool,
}

pub fn load_config() -> Result<Config> {
    let config_path = get_config_dot_path()?.path;
    if !config_path.try_exists().unwrap_or(true) {
        return Err(Error::msg("You have to create a config file with \"dot-manager setup\" first!"))
    }
    read_config(&config_path)
}

pub fn read_config(file_path: &Utf8Path) -> Result<Config> {
    let config_str = std::fs::read_to_string(file_path)?;
    let config = toml::from_str(&config_str)?;
    Ok(config)
}

pub fn write_config(file_path: &Utf8Path, config: &Config) -> Result<()> {
    let config_str = toml::to_string_pretty(config)?;
    std::fs::create_dir_all(file_path.parent().ok_or(Error::msg("Could not create parent directories"))?)?;
    std::fs::write(file_path, config_str)?;
    Ok(())
}