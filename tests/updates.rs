#![cfg(unix)]

use std::{
    fs,
    os::unix::fs::PermissionsExt as _,
    path::Path,
    sync::{Mutex, MutexGuard},
};

use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;

// These subprocess/PTY fixtures exercise a real two-second deadline. Serialize
// them so concurrent cold script launches do not exhaust a sibling's deadline
// on loaded hosts. Keep the production timeout and every assertion unchanged.
static SUBPROCESS_TEST: Mutex<()> = Mutex::new(());

struct Fixture {
    dir: tempfile::TempDir,
    _serial: MutexGuard<'static, ()>,
}

impl Fixture {
    fn new(body: &str, status: u8) -> Self {
        let serial = SUBPROCESS_TEST
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("curl");
        // The fake client never reaches the network. Record invocation arguments.
        fs::write(&path, format!(
            "#!/bin/sh\nprintf '%s\\n' \"$@\" > \"$LOCAL_SEARCH_CONFIG_DIR/curl-called\"\nprintf '%s' '{body}'\nexit {status}\n"
        )).unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
        Self {
            dir,
            _serial: serial,
        }
    }

    fn path(&self) -> &Path {
        self.dir.path()
    }

    fn configure(&self, cmd: &mut Command) {
        cmd.env("PATH", self.path())
            .env("LOCAL_SEARCH_CONFIG_DIR", self.path())
            .env("NO_COLOR", "1")
            .env_remove("CI")
            .env_remove("LOCAL_SEARCH_CDP")
            .env_remove("LOCAL_SEARCH_NO_UPDATE_CHECK");
    }

    fn cli(&self, binary: &str) -> Command {
        let mut cmd = Command::cargo_bin(binary).unwrap();
        self.configure(&mut cmd);
        cmd
    }

    #[cfg(target_os = "macos")]
    fn terminal(&self) -> Command {
        let mut cmd = Command::new("/usr/bin/script");
        cmd.args(["-q", "/dev/null"])
            .arg(assert_cmd::cargo::cargo_bin("lsearch"));
        self.configure(&mut cmd);
        cmd
    }
}

const NEW_RELEASE: &str = r#"{"versions":[{"num":"9999.0.0","yanked":false}]}"#;

#[test]
fn manual_check_returns_only_json_with_a_recommendation_for_every_alias() {
    let fixture = Fixture::new(NEW_RELEASE, 0);
    for binary in ["lsearch", "local-search", "local-browser"] {
        let result = fixture
            .cli(binary)
            .args(["update-check", "--pretty"])
            .assert()
            .success()
            .stderr("")
            .get_output()
            .clone();
        let value: Value = serde_json::from_slice(&result.stdout).unwrap();
        assert_eq!(value["ok"], true);
        assert_eq!(value["update"]["current"], env!("CARGO_PKG_VERSION"));
        assert_eq!(value["update"]["available"], true);
        assert_eq!(
            value["update"]["install_command"],
            "cargo install local-search --locked --force"
        );
    }
    let args = fs::read_to_string(fixture.path().join("curl-called")).unwrap();
    assert!(args.starts_with("-q\n"));
    assert!(args.contains("https://crates.io/api/v1/crates/local-search"));
    assert!(!fixture.path().join("config.json").exists());
    assert!(!fixture.path().join("chrome-profile").exists());
}

#[test]
fn current_release_does_not_recommend_reinstalling() {
    let body = format!(
        r#"{{"versions":[{{"num":"{}","yanked":false}}]}}"#,
        env!("CARGO_PKG_VERSION")
    );
    let fixture = Fixture::new(&body, 0);
    let result = fixture
        .cli("lsearch")
        .arg("update-check")
        .assert()
        .success()
        .stderr("")
        .get_output()
        .clone();
    let value: Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(value["update"]["available"], false);
    assert!(value["update"]["install_command"].is_null());
}

#[test]
fn npm_bridge_checks_its_wrapper_release_and_recommends_npm() {
    let fixture = Fixture::new(
        r#"{"name":"@kevinliu01/localsearch","version":"9999.0.0"}"#,
        0,
    );
    let package = fixture.path().join("package");
    let bin = package.join("vendor/bin");
    fs::create_dir_all(&bin).unwrap();
    fs::write(
        package.join("package.json"),
        r#"{"name":"@kevinliu01/localsearch","version":"0.1.5"}"#,
    )
    .unwrap();
    fs::copy(assert_cmd::cargo::cargo_bin("lsearch"), bin.join("lsearch")).unwrap();
    let mut cmd = Command::new(bin.join("lsearch"));
    fixture.configure(&mut cmd);
    let result = cmd
        .arg("update-check")
        .assert()
        .success()
        .stderr("")
        .get_output()
        .clone();
    let value: Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(value["update"]["source"], "npm");
    assert_eq!(value["update"]["current"], "0.1.5");
    assert_eq!(
        value["update"]["install_command"],
        "npm install -g @kevinliu01/localsearch@latest"
    );
    assert!(
        fs::read_to_string(fixture.path().join("curl-called"))
            .unwrap()
            .contains("registry.npmjs.org/@kevinliu01%2flocalsearch/latest")
    );
}

#[test]
fn explicit_check_works_despite_automatic_opt_out() {
    let fixture = Fixture::new(NEW_RELEASE, 0);
    fixture
        .cli("lsearch")
        .env("LOCAL_SEARCH_NO_UPDATE_CHECK", "1")
        .env("CI", "true")
        .arg("update-check")
        .assert()
        .success();
    assert!(fixture.path().join("curl-called").exists());
}

#[test]
fn missing_curl_is_an_explicit_check_error_not_a_panic() {
    let fixture = Fixture::new(NEW_RELEASE, 0);
    fs::remove_file(fixture.path().join("curl")).unwrap();
    fixture
        .cli("lsearch")
        .arg("update-check")
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains("update_check_failed"))
        .stderr(predicate::str::contains("curl is unavailable"));
}

#[test]
fn stalled_registry_check_has_a_bounded_timeout() {
    let fixture = Fixture::new(NEW_RELEASE, 0);
    fs::write(
        fixture.path().join("curl"),
        "#!/bin/sh\nexec /bin/sleep 20\n",
    )
    .unwrap();
    let start = std::time::Instant::now();
    fixture
        .cli("lsearch")
        .arg("update-check")
        .timeout(std::time::Duration::from_secs(5))
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains("timed out"));
    assert!(start.elapsed() < std::time::Duration::from_secs(5));
}

#[test]
fn registry_failures_have_stable_errors_and_never_claim_up_to_date() {
    for (body, status) in [
        ("offline", 7),
        ("broken json", 0),
        (r#"{"versions":[]}"#, 0),
    ] {
        let fixture = Fixture::new(body, status);
        let result = fixture
            .cli("lsearch")
            .arg("update-check")
            .assert()
            .failure()
            .stdout("")
            .get_output()
            .clone();
        let error: Value = serde_json::from_slice(&result.stderr).unwrap();
        assert_eq!(error["ok"], false);
        assert_eq!(error["error"]["code"], "update_check_failed");
    }
}

#[test]
fn pipes_help_version_and_machine_failures_never_check_automatically() {
    let fixture = Fixture::new(NEW_RELEASE, 0);
    for args in [vec![], vec!["--help"], vec!["--version"], vec!["--json"]] {
        fixture
            .cli("lsearch")
            .args(args)
            .assert()
            .success()
            .stderr("");
    }
    fixture
        .cli("lsearch")
        .args(["search", "test", "--json"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("browser_not_configured"));
    assert!(!fixture.path().join("curl-called").exists());
    assert!(!fixture.path().join("update-check-cargo.json").exists());
}

#[cfg(target_os = "macos")]
#[test]
fn interactive_welcome_recommends_then_reuses_daily_cache() {
    let fixture = Fixture::new(NEW_RELEASE, 0);
    fixture
        .terminal()
        .assert()
        .success()
        .stdout(predicate::str::contains("Update available"));
    fs::remove_file(fixture.path().join("curl-called")).unwrap();
    fixture
        .terminal()
        .assert()
        .success()
        .stdout(predicate::str::contains("Update available"));
    assert!(!fixture.path().join("curl-called").exists());
}

#[cfg(target_os = "macos")]
#[test]
fn interactive_opt_out_and_json_modes_never_check() {
    let fixture = Fixture::new(NEW_RELEASE, 0);
    fixture
        .terminal()
        .env("LOCAL_SEARCH_NO_UPDATE_CHECK", "1")
        .assert()
        .success();
    fixture.terminal().env("CI", "true").assert().success();
    fixture.terminal().arg("--json").assert().success();
    fixture.terminal().arg("--pretty").assert().success();
    assert!(!fixture.path().join("curl-called").exists());
}

#[cfg(target_os = "macos")]
#[test]
fn offline_interactive_startup_remains_successful_and_quiet() {
    let fixture = Fixture::new("offline", 7);
    fixture
        .terminal()
        .assert()
        .success()
        .stdout(predicate::str::contains("Local Browser API for Agents"))
        .stdout(predicate::str::contains("update check failed").not())
        .stdout(predicate::str::contains("Update available").not());
}
