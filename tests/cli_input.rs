use std::{fs, path::PathBuf};

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

#[test]
fn reads_stdin_when_dash_is_supplied() {
    Command::cargo_bin("sdlint")
        .unwrap()
        .arg("-")
        .write_stdin(r#"{"@context":"https://schema.org","@type":"Article"}"#)
        .assert()
        .success()
        .stderr("");
}

#[test]
fn duplicate_stdin_is_an_execution_error() {
    Command::cargo_bin("sdlint")
        .unwrap()
        .args(["-", "-"])
        .write_stdin("{}")
        .assert()
        .code(2)
        .stderr(predicate::str::contains(
            "standard input was specified more than once",
        ));
}

#[test]
fn expands_a_quoted_glob() {
    let directory = tempdir().unwrap();
    fs::write(
        directory.path().join("input.json"),
        r#"{"@context":"https://schema.org","@type":"Article"}"#,
    )
    .unwrap();
    let pattern = format!("{}/*.json", directory.path().display());
    Command::cargo_bin("sdlint")
        .unwrap()
        .arg(pattern)
        .assert()
        .success();
}

#[test]
fn unmatched_glob_is_an_execution_error() {
    Command::cargo_bin("sdlint")
        .unwrap()
        .arg("missing/**/*.json")
        .assert()
        .code(2)
        .stderr(predicate::str::contains("matched no supported files"));
}

#[test]
fn lints_json_ld_fixture_with_text_output() {
    Command::cargo_bin("sdlint")
        .unwrap()
        .arg(fixture("valid.json"))
        .assert()
        .success()
        .stdout("")
        .stderr("");
}

#[test]
fn reports_an_error_diagnostic_and_returns_one() {
    Command::cargo_bin("sdlint")
        .unwrap()
        .arg(fixture("missing-context.json"))
        .assert()
        .code(1)
        .stdout(predicate::str::contains("core/jsonld-context-required"))
        .stderr("");
}

#[test]
fn json_output_contains_diagnostics() {
    Command::cargo_bin("sdlint")
        .unwrap()
        .args(["--format", "json"])
        .arg(fixture("missing-context.json"))
        .assert()
        .code(1)
        .stdout(predicate::str::contains(
            "\"rule_id\":\"core/jsonld-context-required\"",
        ));
}

#[test]
fn fail_on_controls_warning_exit_status() {
    Command::cargo_bin("sdlint")
        .unwrap()
        .args(["--fail-on", "warning"])
        .arg(fixture("missing-type.json"))
        .assert()
        .code(1)
        .stdout(predicate::str::contains("core/jsonld-type-recommended"));
}

#[test]
fn severity_filters_warning_output() {
    Command::cargo_bin("sdlint")
        .unwrap()
        .args(["--severity", "error"])
        .arg(fixture("missing-type.json"))
        .assert()
        .success()
        .stdout("");
}

#[test]
fn parses_json_ld_from_html() {
    Command::cargo_bin("sdlint")
        .unwrap()
        .arg(fixture("valid.html"))
        .assert()
        .success()
        .stderr("");
}

#[test]
fn accepts_multiple_inputs() {
    Command::cargo_bin("sdlint")
        .unwrap()
        .args([fixture("valid.json"), fixture("valid.html")])
        .assert()
        .success()
        .stdout("")
        .stderr("");
}

#[test]
fn malformed_json_is_an_execution_error() {
    Command::cargo_bin("sdlint")
        .unwrap()
        .arg(fixture("invalid.json"))
        .assert()
        .code(2)
        .stderr(predicate::str::contains("cannot parse"));
}
