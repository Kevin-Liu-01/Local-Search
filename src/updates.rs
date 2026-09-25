//! Optional release recommendations. Never participates in browser/search work.

use std::{
    future::Future,
    io::{self, IsTerminal as _},
    path::{Path, PathBuf},
    process::Stdio,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use semver::Version;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::io::AsyncReadExt as _;

use crate::{
    cli::{Cli, Command},
    config,
    error::{Error, Result},
    output, ui,
};

const CHECK_INTERVAL: u64 = 24 * 60 * 60;
const MAX_RESPONSE: u64 = 256 * 1024;
const REQUEST_TIMEOUT: Duration = Duration::from_secs(2);

struct Package {
    name: &'static str,
    current: Version,
    registry: &'static str,
    url: &'static str,
    command: &'static str,
    cache_name: &'static str,
}

impl Package {
    fn cargo() -> Self {
        Self {
            name: "local-search",
            current: Version::parse(env!("CARGO_PKG_VERSION")).expect("valid crate version"),
            registry: "crates.io",
            url: "https://crates.io/api/v1/crates/local-search",
            command: "cargo install local-search --locked --force",
            cache_name: "update-check-cargo.json",
        }
    }

    fn installed() -> Self {
        // The npm bridge keeps the executable at PACKAGE/vendor/bin/lsearch.
        // Read that package only; do not inspect npm config or global installations.
        let npm_version = std::env::current_exe()
            .ok()
            .and_then(|exe| npm_version(&exe));
        if let Some(current) = npm_version {
            Self {
                name: "@kevinliu01/localsearch",
                current,
                registry: "npm",
                url: "https://registry.npmjs.org/@kevinliu01%2flocalsearch/latest",
                command: "npm install -g @kevinliu01/localsearch@latest",
                cache_name: "update-check-npm.json",
            }
        } else {
            Self::cargo()
        }
    }

    fn latest(&self, body: &[u8]) -> Option<Version> {
        let value: Value = serde_json::from_slice(body).ok()?;
        if self.registry == "npm" {
            if value.get("name")?.as_str()? != self.name {
                return None;
            }
            return stable_version(value.get("version")?.as_str()?);
        }
        value
            .get("versions")?
            .as_array()?
            .iter()
            .filter_map(|release| {
                if release.get("yanked")?.as_bool()? {
                    return None;
                }
                stable_version(release.get("num")?.as_str()?)
            })
            .max()
    }
}

fn npm_version(exe: &Path) -> Option<Version> {
    let bin = exe.parent()?;
    let vendor = bin.parent()?;
    if bin.file_name()? != "bin" || vendor.file_name()? != "vendor" {
        return None;
    }
    let raw = std::fs::read(vendor.parent()?.join("package.json")).ok()?;
    let package: Value = serde_json::from_slice(&raw).ok()?;
    if package.get("name")?.as_str()? != "@kevinliu01/localsearch" {
        return None;
    }
    Version::parse(package.get("version")?.as_str()?).ok()
}

fn stable_version(raw: &str) -> Option<Version> {
    let version = Version::parse(raw).ok()?;
    (version.pre.is_empty() && version.build.is_empty()).then_some(version)
}

#[derive(Deserialize, Serialize)]
struct Cache {
    checked_at: u64,
    current: String,
    latest: Option<String>,
}

impl Cache {
    fn fresh(&self, now: u64, current: &Version) -> bool {
        self.current == current.to_string()
            && now
                .checked_sub(self.checked_at)
                .is_some_and(|age| age < CHECK_INTERVAL)
    }
}

pub(crate) fn automatic_check_allowed(cli: &Cli) -> bool {
    let startup = match &cli.command {
        None => cli.query.is_empty(),
        Some(Command::Connect(_) | Command::Launch(_)) => true,
        _ => false,
    };
    startup
        && !cli.json
        && !cli.pretty
        && io::stdout().is_terminal()
        && io::stderr().is_terminal()
        && std::env::var_os("CI").is_none()
        && std::env::var_os("LOCAL_SEARCH_NO_UPDATE_CHECK").is_none()
}

fn cache_path(package: &Package) -> Option<PathBuf> {
    config::config_dir()
        .ok()
        .map(|dir| dir.join(package.cache_name))
}

async fn check_with<F: Future<Output = Result<Vec<u8>>>>(
    package: &Package,
    path: Option<&Path>,
    now: u64,
    force: bool,
    fetch: F,
) -> Result<Option<Version>> {
    if !force
        && let Some(path) = path
        && let Ok(raw) = tokio::fs::read(path).await
        && let Ok(cache) = serde_json::from_slice::<Cache>(&raw)
        && cache.fresh(now, &package.current)
    {
        // A failed attempt also cools down; it must not claim we're up to date.
        return Ok(cache.latest.as_deref().and_then(stable_version));
    }

    let latest = match fetch.await {
        Ok(body) => package.latest(&body).ok_or_else(|| {
            Error::UpdateCheck("registry did not return a valid stable release".to_owned())
        }),
        Err(error) => Err(error),
    };
    let cache = Cache {
        checked_at: now,
        current: package.current.to_string(),
        latest: latest.as_ref().ok().map(ToString::to_string),
    };
    if let Some(path) = path
        && let Some(parent) = path.parent()
        && tokio::fs::create_dir_all(parent).await.is_ok()
        && let Ok(raw) = serde_json::to_vec(&cache)
    {
        // Advisory cache failures must never affect a successful user command.
        let _ = tokio::fs::write(path, raw).await;
    }
    latest.map(Some)
}

async fn fetch(package: &Package) -> Result<Vec<u8>> {
    let request = async {
        // -q first disables .curlrc. No shell, cookies, browser state, or redirects.
        // Use the system HTTPS client instead of adding TLS to the small CLI.
        let mut child = tokio::process::Command::new("curl")
            .args([
                "-q",
                "--fail",
                "--silent",
                "--proto",
                "=https",
                "--connect-timeout",
                "1",
                "--max-time",
                "2",
                "--max-filesize",
                "262144",
                "--user-agent",
                "local-search-update-check (https://github.com/Kevin-Liu-01/Local-Search)",
                package.url,
            ])
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .kill_on_drop(true)
            .spawn()
            .map_err(|_| {
                Error::UpdateCheck("curl is unavailable; install curl to check releases".to_owned())
            })?;
        let mut body = Vec::new();
        child
            .stdout
            .take()
            .expect("piped stdout")
            .take(MAX_RESPONSE + 1)
            .read_to_end(&mut body)
            .await
            .map_err(|_| Error::UpdateCheck("could not read registry response".to_owned()))?;
        if body.len() as u64 > MAX_RESPONSE {
            return Err(Error::UpdateCheck(
                "registry response is too large".to_owned(),
            ));
        }
        let status = child
            .wait()
            .await
            .map_err(|_| Error::UpdateCheck("registry request failed".to_owned()))?;
        if !status.success() {
            return Err(Error::UpdateCheck(
                "registry unavailable; try again when online".to_owned(),
            ));
        }
        Ok(body)
    };
    tokio::time::timeout(REQUEST_TIMEOUT, request)
        .await
        .map_err(|_| Error::UpdateCheck("registry request timed out".to_owned()))?
}

async fn check(package: &Package, force: bool) -> Result<Option<Version>> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    check_with(
        package,
        cache_path(package).as_deref(),
        now,
        force,
        fetch(package),
    )
    .await
}

pub(crate) async fn recommend() {
    let package = Package::installed();
    if let Ok(Some(latest)) = check(&package, false).await
        && latest.cmp_precedence(&package.current).is_gt()
    {
        ui::recommend_update(
            package.name,
            &package.current.to_string(),
            &latest.to_string(),
            package.command,
        );
    }
}

pub(crate) async fn print_check(pretty: bool) -> Result<()> {
    let package = Package::installed();
    let latest = check(&package, true)
        .await?
        .expect("fresh check returns a release");
    let available = latest.cmp_precedence(&package.current).is_gt();
    output::print_json(
        &json!({
            "ok": true,
            "update": {
                "package": package.name,
                "source": package.registry,
                "current": package.current.to_string(),
                "latest": latest.to_string(),
                "available": available,
                "install_command": available.then_some(package.command),
            },
        }),
        pretty,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn releases() -> Vec<u8> {
        br#"{"versions":[{"num":"0.1.5","yanked":false}]}"#.to_vec()
    }

    #[test]
    fn only_recommends_valid_non_yanked_stable_versions() {
        let package = Package::cargo();
        let body = br#"{"versions":[
            {"num":"0.1.9","yanked":false},
            {"num":"0.1.10","yanked":false},
            {"num":"0.2.0-beta.1","yanked":false},
            {"num":"0.3.0","yanked":true},
            {"num":"0.4.0+local","yanked":false},
            {"num":"bad\u001b[31m","yanked":false}] }"#;
        assert_eq!(package.latest(body).unwrap().to_string(), "0.1.10");
        assert!(package.latest(b"not json").is_none());
        assert!(package.latest(br#"{"versions":[]}"#).is_none());
        let current = Version::parse("0.2.0+local").unwrap();
        assert!(
            Version::parse("0.2.0")
                .unwrap()
                .cmp_precedence(&current)
                .is_eq()
        );
        assert!(
            Version::parse("0.1.5")
                .unwrap()
                .cmp_precedence(&current)
                .is_lt()
        );
    }

    #[test]
    fn cache_expires_and_is_invalidated_by_binary_updates_and_clock_changes() {
        let current = Package::cargo().current;
        let mut cache = Cache {
            checked_at: 100,
            current: current.to_string(),
            latest: None,
        };
        assert!(cache.fresh(101, &current));
        assert!(!cache.fresh(99, &current));
        assert!(!cache.fresh(100 + CHECK_INTERVAL, &current));
        cache.current = "0.0.1".to_owned();
        assert!(!cache.fresh(101, &current));
    }

    #[tokio::test]
    async fn daily_cache_skips_fetch_and_manual_check_bypasses_it() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("update.json");
        let package = Package::cargo();
        check_with(&package, Some(&path), 100, false, async { Ok(releases()) })
            .await
            .unwrap();
        let cached = check_with(&package, Some(&path), 101, false, async {
            panic!("must not fetch")
        })
        .await
        .unwrap();
        assert_eq!(cached.unwrap().to_string(), "0.1.5");
        let forced = check_with(&package, Some(&path), 101, true, async {
            Ok(br#"{"versions":[{"num":"0.1.6","yanked":false}]}"#.to_vec())
        })
        .await
        .unwrap();
        assert_eq!(forced.unwrap().to_string(), "0.1.6");
    }

    #[tokio::test]
    async fn offline_check_cools_down_without_claiming_current_or_using_stale_notice() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("update.json");
        let package = Package::cargo();
        check_with(&package, Some(&path), 0, false, async { Ok(releases()) })
            .await
            .unwrap();
        let failure = check_with(&package, Some(&path), CHECK_INTERVAL, false, async {
            Err(Error::UpdateCheck("offline".to_owned()))
        })
        .await;
        assert!(failure.is_err());
        assert!(
            check_with(&package, Some(&path), CHECK_INTERVAL + 1, false, async {
                panic!("must cool down")
            })
            .await
            .unwrap()
            .is_none()
        );
        assert!(
            check_with(&package, Some(&path), CHECK_INTERVAL * 2, false, async {
                Ok(releases())
            })
            .await
            .unwrap()
            .is_some()
        );
    }

    #[tokio::test]
    async fn malformed_or_unwritable_cache_does_not_break_check() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("update.json");
        std::fs::write(&path, "broken").unwrap();
        let package = Package::cargo();
        assert!(
            check_with(&package, Some(&path), 100, false, async { Ok(releases()) })
                .await
                .unwrap()
                .is_some()
        );
        assert!(
            check_with(&package, Some(dir.path()), 100, false, async {
                Ok(releases())
            })
            .await
            .unwrap()
            .is_some()
        );
    }

    #[test]
    fn npm_bridge_is_identified_by_its_own_package_not_the_native_crate_version() {
        let dir = tempfile::tempdir().unwrap();
        let exe = dir.path().join("vendor/bin/lsearch");
        std::fs::write(
            dir.path().join("package.json"),
            br#"{"name":"@kevinliu01/localsearch","version":"0.1.7"}"#,
        )
        .unwrap();
        assert_eq!(npm_version(&exe).unwrap().to_string(), "0.1.7");
        let mut package = Package::cargo();
        package.registry = "npm";
        package.name = "@kevinliu01/localsearch";
        assert_eq!(
            package
                .latest(br#"{"name":"@kevinliu01/localsearch","version":"0.1.8"}"#)
                .unwrap()
                .to_string(),
            "0.1.8"
        );
        assert!(
            package
                .latest(br#"{"name":"wrong","version":"9.0.0"}"#)
                .is_none()
        );
        assert!(npm_version(&dir.path().join("bin/lsearch")).is_none());
    }
}
