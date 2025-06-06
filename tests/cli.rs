use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use serial_test::serial; // TODO: allow tests to run in parallel

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

#[test]
#[serial]
fn show_help_no_args() -> Result<()> {
    run(&[], "Usage")
}

#[test]
#[serial]
fn put_succeed_two_args() -> Result<()> {
    run(&["put", "key", "value"], "")
}

#[test]
#[serial]
fn get_success() -> Result<()> {
    run(&["get", "key"], "")
}

#[test]
#[serial]
fn list_success() -> Result<()> {
    run(&["list"], "")
}

#[test]
#[serial]
fn delete_success() -> Result<()> {
    run(&["delete", "-n", "key"], "")
}

#[test]
#[serial]
fn put_fail_no_args() -> Result<()> {
    Command::cargo_bin("ds")?
        .arg("put")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "the following required arguments were not provided:",
        ));
    Ok(())
}

#[test]
#[serial]
fn put_fail_single_arg() -> Result<()> {
    Command::cargo_bin("ds")?
        .args(["put", "key"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "the following required arguments were not provided:",
        ));
    Ok(())
}
