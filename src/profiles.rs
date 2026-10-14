//! Named browser profiles, each with its own site policy, and the tabs an agent
//! opened or was granted in the current browser session.
//!
//! Profiles live in `profiles.json` and tab records in `tabs.json`, apart from the
//! saved browser selection in `config.json`, so connecting never rewrites them.

use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{
    config::{self, Config, Connection},
    error::{Error, IoContext, Result},
    policy::Policy,
};

/// Your everyday Chrome, reached with `connect --existing`.
pub const EVERYDAY: &str = "everyday";
/// The default separate profile, reached with `connect --managed`.
pub const AGENT: &str = "agent";
/// An explicit endpoint, reached with `connect <endpoint>` or `--cdp`.
pub const CUSTOM: &str = "custom";
/// The CDP port of the default separate profile.
pub const DEFAULT_MANAGED_PORT: u16 = 9322;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Registry {
    #[serde(default)]
    pub profiles: BTreeMap<String, Profile>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Profile {
    /// How to reach this profile's browser. Built-in profiles may leave it empty
    /// and use their defaults.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connection: Option<Connection>,
    /// The endpoint behind the `custom` profile.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(default)]
    pub policy: Policy,
}

/// Where a profile's browser lives, with built-in defaults filled in.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Browser {
    /// Everyday Chrome; `None` means the default Chrome user-data directory.
    Existing(Option<PathBuf>),
    Managed {
        dir: PathBuf,
        port: u16,
        browser_path: Option<PathBuf>,
    },
    Endpoint(Option<String>),
}

pub fn registry_path() -> Result<PathBuf> {
    Ok(config::config_dir()?.join("profiles.json"))
}

pub fn load() -> Result<Registry> {
    read_json(&registry_path()?)
}

/// Applies `edit` to the registry under an exclusive lock and saves it.
pub fn update<T>(edit: impl FnOnce(&mut Registry) -> Result<T>) -> Result<T> {
    let _lock = exclusive_lock("profiles.lock")?;
    let path = registry_path()?;
    let mut registry: Registry = read_json(&path)?;
    let out = edit(&mut registry)?;
    write_json(&path, &registry)?;
    Ok(out)
}

/// Profile names are short, lowercase and safe as directory names.
pub fn validate_name(name: &str) -> Result<()> {
    let mut chars = name.chars();
    let valid = name.len() <= 32
        && chars
            .next()
            .is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || matches!(c, '-' | '_'));
    if valid {
        Ok(())
    } else {
        Err(Error::InvalidArgument(format!(
            "profile name {name:?} must be 1-32 lowercase letters, digits, - or _, starting with a letter or digit"
        )))
    }
}

/// The browser a profile name refers to, or `None` when no such profile exists.
pub fn browser_of(registry: &Registry, name: &str) -> Result<Option<Browser>> {
    let profile = registry.profiles.get(name);
    if let Some(connection) = profile.and_then(|profile| profile.connection.as_ref()) {
        return Ok(Some(match connection {
            Connection::Existing { profile } => Browser::Existing(Some(profile.clone())),
            Connection::Managed {
                profile,
                port,
                browser_path,
            } => Browser::Managed {
                dir: profile.clone(),
                port: *port,
                browser_path: browser_path.clone(),
            },
            Connection::Endpoint => Browser::Endpoint(profile.and_then(|p| p.endpoint.clone())),
        }));
    }
    Ok(match name {
        EVERYDAY => Some(Browser::Existing(None)),
        AGENT => Some(Browser::Managed {
            dir: config::managed_profile_dir()?,
            port: DEFAULT_MANAGED_PORT,
            browser_path: None,
        }),
        CUSTOM => Some(Browser::Endpoint(profile.and_then(|p| p.endpoint.clone()))),
        _ => None,
    })
}

/// The name of the profile behind a saved connection. A registered profile with
/// the same browser wins; otherwise the built-in profile of that kind.
pub fn name_for(registry: &Registry, connection: Option<&Connection>) -> Option<String> {
    let connection = connection?;
    if let Some((name, _)) = registry.profiles.iter().find(|(_, profile)| {
        profile
            .connection
            .as_ref()
            .is_some_and(|registered| same_browser(registered, connection))
    }) {
        return Some(name.clone());
    }
    Some(
        match connection {
            Connection::Existing { .. } => EVERYDAY,
            // An unregistered `launch --profile DIR` shares the agent profile's rules.
            Connection::Managed { .. } => AGENT,
            Connection::Endpoint => CUSTOM,
        }
        .to_owned(),
    )
}

/// The name and policy that apply to a command. A transient `--cdp` override
/// uses the `custom` profile's rules, whatever is saved.
pub fn active(
    registry: &Registry,
    saved: &Config,
    transient_endpoint: bool,
) -> (Option<String>, Policy) {
    let name = if transient_endpoint {
        Some(CUSTOM.to_owned())
    } else {
        name_for(registry, saved.connection.as_ref())
    };
    let policy = name
        .as_deref()
        .and_then(|name| registry.profiles.get(name))
        .map(|profile| profile.policy.clone())
        .unwrap_or_default();
    (name, policy)
}

/// Two connections reach the same browser profile.
pub fn same_browser(a: &Connection, b: &Connection) -> bool {
    match (a, b) {
        (Connection::Existing { profile: a }, Connection::Existing { profile: b }) => {
            same_dir(a, b)
        }
        (
            Connection::Managed {
                profile: a,
                port: pa,
                ..
            },
            Connection::Managed {
                profile: b,
                port: pb,
                ..
            },
        ) => pa == pb && same_dir(a, b),
        (Connection::Endpoint, Connection::Endpoint) => true,
        _ => false,
    }
}

fn same_dir(a: &Path, b: &Path) -> bool {
    a == b || matches!((a.canonicalize(), b.canonicalize()), (Ok(a), Ok(b)) if a == b)
}

/// The tabs an agent opened, and the tabs a person handed over, in one browser
/// session. Tab ids from an earlier session mean nothing, so a new endpoint
/// starts an empty record.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct TabLedger {
    #[serde(default)]
    endpoint: String,
    #[serde(default)]
    pub owned: BTreeSet<String>,
    #[serde(default)]
    pub granted: BTreeSet<String>,
}

impl TabLedger {
    pub fn allows(&self, target_id: &str) -> bool {
        self.owned.contains(target_id) || self.granted.contains(target_id)
    }
}

fn ledger_path() -> Result<PathBuf> {
    Ok(config::config_dir()?.join("tabs.json"))
}

/// The tab record for the browser at `endpoint`.
pub fn tabs_for(endpoint: &str) -> Result<TabLedger> {
    let ledger: TabLedger = read_json(&ledger_path()?)?;
    Ok(if ledger.endpoint == endpoint {
        ledger
    } else {
        TabLedger::default()
    })
}

/// Applies `edit` to the tab record for `endpoint` under an exclusive lock and saves it.
pub fn update_tabs<T>(endpoint: &str, edit: impl FnOnce(&mut TabLedger) -> T) -> Result<T> {
    let _lock = exclusive_lock("tabs.lock")?;
    let path = ledger_path()?;
    let mut ledger: TabLedger = read_json(&path)?;
    if ledger.endpoint != endpoint {
        ledger = TabLedger {
            endpoint: endpoint.to_owned(),
            ..TabLedger::default()
        };
    }
    let out = edit(&mut ledger);
    write_json(&path, &ledger)?;
    Ok(out)
}

/// Records a tab lsearch created so `tabs own` profiles can keep using it.
pub fn record_owned(endpoint: &str, target_id: &str) -> Result<()> {
    update_tabs(endpoint, |ledger| {
        ledger.owned.insert(target_id.to_owned());
    })
}

fn exclusive_lock(name: &str) -> Result<std::fs::File> {
    let dir = config::config_dir()?;
    std::fs::create_dir_all(&dir).at(config::display_path(&dir))?;
    let path = dir.join(name);
    let mut options = std::fs::OpenOptions::new();
    options.read(true).write(true).create(true).truncate(false);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let file = options.open(&path).at(config::display_path(&path))?;
    // These writes take milliseconds; waiting is better than failing a command.
    file.lock().at(config::display_path(&path))?;
    Ok(file)
}

fn read_json<T: Default + for<'de> Deserialize<'de>>(path: &Path) -> Result<T> {
    match std::fs::read_to_string(path) {
        Ok(raw) if raw.trim().is_empty() => Ok(T::default()),
        Ok(raw) => Ok(serde_json::from_str(&raw)?),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(T::default()),
        Err(source) => Err(Error::Io {
            path: config::display_path(path),
            source,
        }),
    }
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).at(config::display_path(parent))?;
    }
    // Write beside the file and rename, so a reader never sees half a file.
    let partial = path.with_extension("json.partial");
    std::fs::write(&partial, serde_json::to_vec_pretty(value)?)
        .at(config::display_path(&partial))?;
    std::fs::rename(&partial, path).at(config::display_path(path))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn managed(dir: &str, port: u16) -> Connection {
        Connection::Managed {
            profile: PathBuf::from(dir),
            port,
            browser_path: None,
        }
    }

    #[test]
    fn profile_names_are_safe_directory_names() {
        for name in ["research", "work-2", "a", "x_y"] {
            assert!(validate_name(name).is_ok(), "{name}");
        }
        for name in ["", "Research", "-x", "a/b", "../up", "a b", &"x".repeat(33)] {
            assert!(validate_name(name).is_err(), "{name:?}");
        }
    }

    #[test]
    fn connections_map_to_registered_or_built_in_profile_names() {
        let mut registry = Registry::default();
        registry.profiles.insert(
            "research".to_owned(),
            Profile {
                connection: Some(managed("/tmp/ls-research", 9323)),
                ..Profile::default()
            },
        );
        assert_eq!(
            name_for(&registry, Some(&managed("/tmp/ls-research", 9323))).as_deref(),
            Some("research")
        );
        assert_eq!(
            name_for(&registry, Some(&managed("/tmp/ls-research", 9400))).as_deref(),
            Some(AGENT)
        );
        let existing = Connection::Existing {
            profile: PathBuf::from("/tmp/chrome"),
        };
        assert_eq!(
            name_for(&registry, Some(&existing)).as_deref(),
            Some(EVERYDAY)
        );
        assert_eq!(
            name_for(&registry, Some(&Connection::Endpoint)).as_deref(),
            Some(CUSTOM)
        );
        assert_eq!(name_for(&registry, None), None);
    }

    #[test]
    fn the_active_policy_belongs_to_the_profile_behind_the_connection() {
        let mut registry = Registry::default();
        let mut strict = Policy::default();
        strict.block.push("bank.com".to_owned());
        registry.profiles.insert(
            EVERYDAY.to_owned(),
            Profile {
                policy: strict.clone(),
                ..Profile::default()
            },
        );
        let saved = Config {
            connection: Some(Connection::Existing {
                profile: PathBuf::from("/tmp/chrome"),
            }),
            ..Config::default()
        };
        assert_eq!(
            active(&registry, &saved, false),
            (Some(EVERYDAY.to_owned()), strict)
        );
        assert_eq!(
            active(&registry, &saved, true),
            (Some(CUSTOM.to_owned()), Policy::default())
        );
    }

    #[test]
    fn built_in_profiles_resolve_without_registration() {
        let registry = Registry::default();
        assert_eq!(
            browser_of(&registry, EVERYDAY).unwrap(),
            Some(Browser::Existing(None))
        );
        assert!(matches!(
            browser_of(&registry, AGENT).unwrap(),
            Some(Browser::Managed {
                port: DEFAULT_MANAGED_PORT,
                ..
            })
        ));
        assert_eq!(browser_of(&registry, "missing").unwrap(), None);
    }

    #[test]
    fn a_new_browser_session_starts_an_empty_tab_record() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("tabs.json");
        write_json(
            &path,
            &TabLedger {
                endpoint: "ws://one".to_owned(),
                owned: BTreeSet::from(["A".to_owned()]),
                granted: BTreeSet::new(),
            },
        )
        .unwrap();
        let ledger: TabLedger = read_json(&path).unwrap();
        assert!(ledger.allows("A"));
        assert!(!ledger.allows("B"));
        assert_eq!(
            read_json::<TabLedger>(&dir.path().join("missing.json"))
                .unwrap()
                .owned
                .len(),
            0
        );
    }
}
