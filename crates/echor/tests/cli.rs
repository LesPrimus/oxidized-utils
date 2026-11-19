use assert_cmd::Command;
use std::process::Command as StdCommand;

fn matches_echo(args: &[&str]) {
    let expected = StdCommand::new("echo").args(args).output().unwrap().stdout;

    Command::cargo_bin("echor")
        .unwrap()
        .args(args)
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("Usage"));
}

#[test]
fn hello1() {
    matches_echo(&["Hello there"]);
}

#[test]
fn hello2() {
    matches_echo(&["Hello", "there"]);
}

#[test]
fn hello1_no_newline() {
    matches_echo(&["-n", "Hello there"]);
}

#[test]
fn hello2_no_newline() {
    matches_echo(&["-n", "Hello", "there"]);
}
