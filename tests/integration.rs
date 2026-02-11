use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_help_command() {
    let mut cmd = Command::cargo_bin("llm-cli").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("A CLI tool for interacting with LLMs"));
}

#[test]
fn test_version_command() {
    let mut cmd = Command::cargo_bin("llm-cli").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("0.1.0"));
}

#[test]
fn test_config_show() {
    let mut cmd = Command::cargo_bin("llm-cli").unwrap();
    cmd.arg("config")
        .arg("show")
        .assert()
        .success();
}

#[test]
fn test_session_list() {
    let mut cmd = Command::cargo_bin("llm-cli").unwrap();
    cmd.arg("session")
        .arg("list")
        .assert()
        .success();
}