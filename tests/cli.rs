use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
// use pretty_assertions::assert_eq;
// use std::fs;

#[test]
fn show_help_no_args() -> Result<()> {
    run(&[], "Usage")
}

#[test]
fn put_succeed_two_args() -> Result<()> {
    run(&["put", "key", "value"], "")
}

#[test]
fn get_success() -> Result<()> {
    run(&["get", "key"], "")
}

#[test]
fn list_success() -> Result<()> {
    run(&["list"], "")
}

#[test]
fn delete_success() -> Result<()> {
    run(&["delete", "-n", "key"], "")
}

#[test]
fn put_fail_no_args() -> Result<()> {
    Command::cargo_bin("ds")?
        .arg("put")
        .assert()
        .failure()
        .stderr(predicate::str::contains("the following required arguments were not provided:"));
    Ok(())
}

#[test]
fn put_fail_single_arg() -> Result<()> {
    Command::cargo_bin("ds")?
        .args(["put", "key"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("the following required arguments were not provided:"));
    Ok(())
}

// TODO: this currently uses an empty string to test only for success, I don't love that
fn run(args: &[&str], substring: &str) -> Result<()> {
    Command::cargo_bin("ds")?
        .args(args)
        .assert()
        .success()
        .stdout(predicate::str::contains(substring))
        .stderr(predicate::str::is_empty());
    Ok(())
}
