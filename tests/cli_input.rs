use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn reads_stdin_when_dash_is_supplied() {
    Command::cargo_bin("sdlint")
        .unwrap()
        .arg("-")
        .write_stdin("{}")
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
    fs::write(directory.path().join("input.json"), "{}").unwrap();
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
