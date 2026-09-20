use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn no_arguments_show_the_branded_quick_start() {
    let mut cmd = Command::cargo_bin("lsearch").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Local Browser API for Agents"))
        .stdout(predicate::str::contains("--engine google"));
}

#[test]
fn help_includes_search_and_artifact_commands() {
    let mut cmd = Command::cargo_bin("lsearch").unwrap();
    cmd.arg("--help").assert().success().stdout(
        predicate::str::contains("search")
            .and(predicate::str::contains("screenshot"))
            .and(predicate::str::contains("cleanup"))
            .and(predicate::str::contains("mhtml")),
    );
}

#[test]
fn search_help_documents_human_and_agent_output_modes() {
    let mut cmd = Command::cargo_bin("lsearch").unwrap();
    cmd.args(["search", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("--format <FORMAT>"))
        .stdout(predicate::str::contains("auto"))
        .stdout(predicate::str::contains("table"))
        .stdout(predicate::str::contains("json"));
}

#[test]
fn global_help_documents_the_json_override() {
    let mut cmd = Command::cargo_bin("lsearch").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--json"));
}

#[test]
fn invalid_wait_arguments_return_stable_error_json() {
    let mut cmd = Command::cargo_bin("local-browser").unwrap();
    cmd.args(["--cdp", "ws://127.0.0.1:1/devtools/browser/missing", "wait"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("\"ok\":false"));
}
