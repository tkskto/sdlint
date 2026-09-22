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
fn requires_an_input_operand() {
    Command::cargo_bin("sdlint")
        .unwrap()
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("<INPUT>..."));
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
fn reports_missing_article_headline_as_warning() {
    Command::cargo_bin("sdlint")
        .unwrap()
        .arg(fixture("article-missing-headline.json"))
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "google/article/headline-recommended",
        ))
        .stderr("");
}

#[test]
fn article_headline_warning_can_fail_the_run() {
    Command::cargo_bin("sdlint")
        .unwrap()
        .args(["--fail-on", "warning"])
        .arg(fixture("article-missing-headline.json"))
        .assert()
        .code(1)
        .stdout(predicate::str::contains(
            "google/article/headline-recommended",
        ))
        .stderr("");
}

#[test]
fn reports_missing_article_recommended_properties() {
    let output = Command::cargo_bin("sdlint")
        .unwrap()
        .arg(fixture("article-missing-recommended-properties.json"))
        .output()
        .unwrap();

    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let stdout = String::from_utf8(output.stdout).unwrap();

    for rule_id in [
        "schema/article/article-body-present",
        "schema/article/article-section-present",
        "schema/article/backstory-present",
        "schema/article/page-end-present",
        "schema/article/page-start-present",
        "schema/article/pagination-present",
        "schema/article/speakable-present",
        "schema/article/word-count-present",
        "google/article/author-recommended",
        "google/article/date-published-recommended",
        "google/article/date-modified-recommended",
        "google/article/headline-recommended",
        "google/article/image-recommended",
    ] {
        assert!(stdout.contains(rule_id));
    }
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

#[test]
fn continues_linting_after_invalid_json_ld_in_html() {
    let directory = tempdir().unwrap();
    let path = directory.path().join("input.html");
    fs::write(
        &path,
        r#"
            <script type="application/ld+json">invalid</script>
            <script type="application/ld+json">{"@type":"Organization"}</script>
        "#,
    )
    .unwrap();

    Command::cargo_bin("sdlint")
        .unwrap()
        .arg(path)
        .assert()
        .code(2)
        .stdout(predicate::str::contains("core/jsonld-context-required"))
        .stderr(predicate::str::contains("cannot parse"));
}

#[test]
fn loads_sdlint_toml_and_applies_rule_settings() {
    let directory = tempdir().unwrap();
    let article_path = directory.path().join("article.json");
    fs::copy(fixture("article-missing-headline.json"), &article_path).unwrap();
    fs::write(
        directory.path().join("sdlint.toml"),
        r#"
            [rules]
            "google/article/headline-recommended" = "error"
        "#,
    )
    .unwrap();

    Command::cargo_bin("sdlint")
        .unwrap()
        .current_dir(directory.path())
        .arg("article.json")
        .assert()
        .code(1)
        .stdout(predicate::str::contains(
            "error google/article/headline-recommended",
        ));
}

#[test]
fn command_line_fail_on_takes_precedence_over_configuration() {
    let directory = tempdir().unwrap();
    let article_path = directory.path().join("article.json");
    fs::copy(fixture("article-missing-headline.json"), &article_path).unwrap();
    fs::write(
        directory.path().join("sdlint.toml"),
        r#"
            [exit]
            fail_on = "warning"
        "#,
    )
    .unwrap();

    Command::cargo_bin("sdlint")
        .unwrap()
        .current_dir(directory.path())
        .args(["--fail-on", "none", "article.json"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "google/article/headline-recommended",
        ));
}

#[test]
fn configured_fail_on_controls_warning_exit_status() {
    let directory = tempdir().unwrap();
    fs::copy(
        fixture("article-missing-headline.json"),
        directory.path().join("article.json"),
    )
    .unwrap();
    fs::write(
        directory.path().join("sdlint.toml"),
        r#"
            [exit]
            fail_on = "warning"
        "#,
    )
    .unwrap();

    Command::cargo_bin("sdlint")
        .unwrap()
        .current_dir(directory.path())
        .arg("article.json")
        .assert()
        .code(1)
        .stdout(predicate::str::contains(
            "google/article/headline-recommended",
        ));
}

#[test]
fn applies_path_specific_rule_override() {
    let directory = tempdir().unwrap();
    let fixture_directory = directory.path().join("fixtures");
    fs::create_dir(&fixture_directory).unwrap();
    fs::copy(
        fixture("article-missing-headline.json"),
        fixture_directory.join("article.json"),
    )
    .unwrap();
    fs::write(
        directory.path().join("sdlint.toml"),
        r#"
            [rules]
            "google/article/headline-recommended" = "error"

            [[overrides]]
            files = ["fixtures/*.json"]
            rules = { "google/article/headline-recommended" = "off" }
        "#,
    )
    .unwrap();

    Command::cargo_bin("sdlint")
        .unwrap()
        .current_dir(directory.path())
        .arg("fixtures/article.json")
        .assert()
        .success()
        .stdout("");
}

#[test]
fn silently_ignores_a_file_matching_configured_patterns() {
    let directory = tempdir().unwrap();
    fs::write(directory.path().join("ignored.json"), "invalid").unwrap();
    fs::write(
        directory.path().join("sdlint.toml"),
        r#"
            [files]
            ignore = ["ignored.json"]
        "#,
    )
    .unwrap();

    Command::cargo_bin("sdlint")
        .unwrap()
        .current_dir(directory.path())
        .arg("ignored.json")
        .assert()
        .success()
        .stdout("")
        .stderr("");
}

#[test]
fn respects_gitignore_when_enabled() {
    let directory = tempdir().unwrap();
    fs::write(directory.path().join("ignored.json"), "invalid").unwrap();
    fs::write(directory.path().join(".gitignore"), "ignored.json\n").unwrap();
    fs::write(
        directory.path().join("sdlint.toml"),
        r#"
            [files]
            respect_gitignore = true
        "#,
    )
    .unwrap();

    Command::cargo_bin("sdlint")
        .unwrap()
        .current_dir(directory.path())
        .arg("ignored.json")
        .assert()
        .success()
        .stdout("")
        .stderr("");
}

#[test]
fn rejects_unknown_configuration_keys() {
    let directory = tempdir().unwrap();
    fs::write(
        directory.path().join("sdlint.toml"),
        r#"
            [files]
            unknown = true
        "#,
    )
    .unwrap();

    Command::cargo_bin("sdlint")
        .unwrap()
        .current_dir(directory.path())
        .arg(fixture("valid.json"))
        .assert()
        .code(2)
        .stdout("")
        .stderr(predicate::str::contains("unknown field `unknown`"));
}

#[test]
fn rejects_unknown_rule_ids() {
    let directory = tempdir().unwrap();
    fs::write(
        directory.path().join("sdlint.toml"),
        r#"
            [rules]
            "google/article/typo" = "off"
        "#,
    )
    .unwrap();

    Command::cargo_bin("sdlint")
        .unwrap()
        .current_dir(directory.path())
        .arg(fixture("valid.json"))
        .assert()
        .code(2)
        .stdout("")
        .stderr(predicate::str::contains("unknown Rule ID"));
}
