use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_help() {
    let output = Command::new("target/debug/cli-calc")
        .arg("-h")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Rust command-line calculator"));
}

#[test]
fn test_version() {
    let output = Command::new("target/debug/cli-calc")
        .arg("--version")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("cli-calc 1.0"));
}

#[test]
fn test_about() {
    let output = Command::new("target/debug/cli-calc")
        .arg("--about")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Rust command-line calculator"));
    assert!(stdout.contains("Author: Srikanth Kotturi"));
}

#[test]
fn test_addition() {
    let mut cmd = Command::cargo_bin("cli-calc").unwrap();
    cmd.args(&["-o", "add", "--o1", "2", "--o2", "4"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Result: 6"));
}

#[test]
fn test_addition_with_symbol() {
    let mut cmd = Command::cargo_bin("cli-calc").unwrap();
    cmd.args(&["-o", "+", "--o1", "2", "--o2", "4"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Result: 6"));
}

#[test]
fn test_subtraction() {
    let mut cmd = Command::cargo_bin("cli-calc").unwrap();
    cmd.args(&["-o", "sub", "--o1", "3", "--o2", "2"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Result: 1"));
}

#[test]
fn test_missing_operation() {
    let mut cmd = Command::cargo_bin("cli-calc").unwrap();
    cmd.args(&["--o1", "4", "--o2", "4"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("No operation specified. Exiting."));
}
