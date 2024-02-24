use std::env::current_dir;
use std::str::FromStr;

use anyhow::Error;
use camino::Utf8PathBuf;
use crate::config::Config;

#[derive(Debug)]
pub struct DotPath {
    pub relative_to_home: String,
    pub path: Utf8PathBuf,
    pub path_in_synced_folder: Utf8PathBuf,
}

pub fn get_home_dir() -> Option<Utf8PathBuf> {
    Some(Utf8PathBuf::from_path_buf(dirs::home_dir()?).ok()?.canonicalize_utf8().ok()?)
}

pub fn get_absolute_path(path: &String, home_dir: &Utf8PathBuf) -> anyhow::Result<Utf8PathBuf> {
    let path = path.strip_suffix("/").unwrap_or(path.as_str());
    if let Some(path_stripped) = path.strip_prefix("~/") {
        Ok(home_dir.join(path_stripped))
    } else if path.starts_with("/") {
        Ok(Utf8PathBuf::from_str(&*path).map_err(|_| Error::msg("Failed to parse path"))?)
    } else {
        Ok(Utf8PathBuf::from_path_buf(
            current_dir()
                .map_err(|_| Error::msg("Couldn't get current directory to resolve relative path!"))?
        )
            .map_err(|_| Error::msg("Failed to parse current directory!"))?
            .join(path))
    }
}

pub fn get_dot_path(
    config: &Config,
    path: &String,
) -> anyhow::Result<DotPath> {
    let home_dir = get_home_dir().ok_or(Error::msg("Couldn't detect home dir"))?;
    let path = get_absolute_path(&path, &home_dir)?;
    let synced_folder = get_absolute_path(&config.synced_folder, &home_dir)?;
    let relative_to_home = path
        .strip_prefix(home_dir)
        .map_err(|_| Error::msg("failed to determine relative to home dir path"))?;
    let path_in_synced_folder = synced_folder.join(relative_to_home);
    return Ok(DotPath {
        relative_to_home: relative_to_home.as_str().to_string(),
        path,
        path_in_synced_folder,
    });
}
