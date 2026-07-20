use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{IoContext, Result};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub endpoint: Option<String>,
    pub target_id: Option<String>,
}

pub fn config_dir() -> Result<PathBuf> {
    Ok(dirs::config_dir()
        .ok_or_else(|| {
            crate::error::Error::InvalidArgument("cannot resolve config dir".to_owned())
        })?
        .join("local-browser"))
}

pub fn config_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("config.json"))
}

pub fn managed_profile_dir() -> Result<PathBuf> {
    Ok(config_dir()?.join("chrome-profile"))
}

pub fn managed_devtools_file() -> Result<PathBuf> {
    Ok(managed_profile_dir()?.join("DevToolsActivePort"))
}

pub fn display_path(path: &Path) -> String {
    path.display().to_string()
}

pub async fn load() -> Result<Config> {
    let path = config_path()?;
    if !path.exists() {
        return Ok(Config::default());
    }
    let raw = tokio::fs::read_to_string(&path)
        .await
        .map_err(|source| crate::error::Error::Io {
            path: display_path(&path),
            source,
        })?;
    Ok(serde_json::from_str(&raw)?)
}

pub async fn save(config: &Config) -> Result<PathBuf> {
    let path = config_path()?;
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|source| crate::error::Error::Io {
                path: display_path(parent),
                source,
            })?;
    }
    let raw = serde_json::to_vec_pretty(config)?;
    std::fs::write(&path, raw).at(path.display().to_string())?;
    Ok(path)
}
