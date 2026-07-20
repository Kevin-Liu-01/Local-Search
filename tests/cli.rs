use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn help_includes_search_and_artifact_commands() {
    let mut cmd = Command::cargo_bin("local-browser").unwrap();
    cmd.arg("--help").assert().success().stdout(
        predicate::str::contains("search")
            .and(predicate::str::contains("screenshot"))
            .and(predicate::str::contains("mhtml")),
    );
}

#[test]
fn invalid_wait_arguments_return_stable_error_json() {
    let mut cmd = Command::cargo_bin("local-browser").unwrap();
    cmd.args(["--cdp", "ws://127.0.0.1:1/devtools/browser/missing", "wait"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("\"ok\":false"));
}
