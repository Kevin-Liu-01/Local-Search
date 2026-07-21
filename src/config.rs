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
        .join("local-search"))
}

pub fn config_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("config.json"))
}

fn legacy_config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|dir| dir.join("local-browser/config.json"))
}

pub fn managed_profile_dir() -> Result<PathBuf> {
    Ok(config_dir()?.join("chrome-profile"))
}

pub fn managed_devtools_file() -> Result<PathBuf> {
    Ok(managed_profile_dir()?.join("DevToolsActivePort"))
}

pub fn managed_pid_file() -> Result<PathBuf> {
    Ok(config_dir()?.join("managed-chrome.pid"))
}

pub fn search_cache_dir() -> Result<PathBuf> {
    if let Some(path) = std::env::var_os("LOCAL_SEARCH_CACHE_DIR") {
        return Ok(PathBuf::from(path));
    }
    Ok(dirs::cache_dir()
        .ok_or_else(|| crate::error::Error::InvalidArgument("cannot resolve cache dir".to_owned()))?
        .join("local-search/searches"))
}

pub fn display_path(path: &Path) -> String {
    path.display().to_string()
}

pub async fn load() -> Result<Config> {
    let path = config_path()?;
    let path = if path.exists() {
        path
    } else if let Some(legacy) = legacy_config_path() {
        if legacy.exists() {
            legacy
        } else {
            return Ok(Config::default());
        }
    } else {
        return Ok(Config::default());
    };
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
