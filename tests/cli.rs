use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
// use pretty_assertions::assert_eq;
// use std::fs;

#[test]
fn show_help_no_args() -> Result<()> {
    let mut cmd = Command::cargo_bin("ds").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Usage"));
    Ok(())
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

#[test]
fn put_succeed_two_args() -> Result<()> {
    Command::cargo_bin("ds")?
        .args(["put", "key", "value"])
        .assert()
        .success();
    Ok(())
}

// TODO: write a run function to avoid redundancy
