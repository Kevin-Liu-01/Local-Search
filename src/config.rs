use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{IoContext, Result};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub endpoint: Option<String>,
    pub target_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connection: Option<Connection>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session: Option<BrowserSession>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct BrowserSession {
    pub directory: PathBuf,
    pub endpoint: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum Connection {
    Existing {
        profile: PathBuf,
    },
    Managed {
        profile: PathBuf,
        port: u16,
        browser_path: Option<PathBuf>,
    },
    Endpoint,
}

pub fn config_dir() -> Result<PathBuf> {
    if let Some(path) = std::env::var_os("LOCAL_SEARCH_CONFIG_DIR") {
        return Ok(PathBuf::from(path));
    }
    Ok(dirs::config_dir()
        .ok_or_else(|| {
            crate::error::Error::InvalidArgument("cannot resolve config dir".to_owned())
        })?
        .join("local-search"))
}

pub fn config_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("config.json"))
}

/// Serialize selection changes; the OS releases this lock when a process exits.
pub fn selection_lock() -> Result<std::fs::File> {
    let path = config_dir()?.join("selection.lock");
    std::fs::create_dir_all(config_dir()?).at("browser selection directory")?;
    let mut options = std::fs::OpenOptions::new();
    options.read(true).write(true).create(true).truncate(false);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let file = options.open(&path).at(path.display().to_string())?;
    file.try_lock().map_err(|error| match error {
        std::fs::TryLockError::WouldBlock => crate::error::Error::BrowserBusy,
        std::fs::TryLockError::Error(source) => crate::error::Error::Io {
            path: path.display().to_string(),
            source,
        },
    })?;
    Ok(file)
}

fn legacy_config_path() -> Option<PathBuf> {
    if std::env::var_os("LOCAL_SEARCH_CONFIG_DIR").is_some() {
        return None;
    }
    dirs::config_dir().map(|dir| dir.join("local-browser/config.json"))
}

pub fn managed_profile_dir() -> Result<PathBuf> {
    Ok(config_dir()?.join("chrome-profile"))
}

pub fn managed_devtools_file() -> Result<PathBuf> {
    Ok(managed_profile_dir()?.join("DevToolsActivePort"))
}

pub fn managed_pid_file(port: u16) -> Result<PathBuf> {
    let name = if port == 9322 {
        "managed-chrome.pid".to_owned()
    } else {
        format!("managed-chrome-{port}.pid")
    };
    Ok(config_dir()?.join(name))
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

#[cfg(test)]
mod tests {
    use super::managed_pid_file;

    #[test]
    fn managed_pid_files_are_scoped_to_non_default_ports() {
        assert_eq!(
            managed_pid_file(9322)
                .unwrap()
                .file_name()
                .unwrap()
                .to_string_lossy(),
            "managed-chrome.pid"
        );
        assert_eq!(
            managed_pid_file(9444)
                .unwrap()
                .file_name()
                .unwrap()
                .to_string_lossy(),
            "managed-chrome-9444.pid"
        );
    }
}
