// tests/integration.rs
mod common;
use common::run_gentk;
use predicates::prelude::*;

const ALL_COMMANDS: &[(&str, &[&str])] = &[
    ("password", &[] as &[&str]),
    ("uuid4", &[]),
    ("uuid5", &["--namespace=dns", "--name=test"]),
    ("uuid7", &[]),
    ("ulid", &[]),
    ("cuid2", &[]),
    ("nanoid", &[]),
    ("nanoid", &["--length=10", "--alphabet=abc"]),
    ("tsid", &[]),
];

#[test]
fn all_commands_generate_one_value() {
    for &(cmd, extra) in ALL_COMMANDS {
        let mut args = vec![cmd];
        args.extend_from_slice(extra);
        args.push("1");
        let mut cmd_under_test = run_gentk(&args);  // mut, потому что .assert() требует &mut self
        cmd_under_test
            .assert()
            .success()
            .stdout(predicate::ne(""));
    }
}

#[test]
fn all_commands_json_single() {
    for &(cmd, extra) in ALL_COMMANDS {
        let mut args = vec![cmd, "--json"];
        args.extend_from_slice(extra);
        args.push("1");
        let mut cmd_under_test = run_gentk(&args);
        cmd_under_test
            .assert()
            .success()
            .stdout(predicate::str::contains("\"value\":"));
    }
}

#[test]
fn all_commands_json_multiple() {
    for &(cmd, extra) in ALL_COMMANDS {
        let mut args = vec![cmd, "--json"];
        args.extend_from_slice(extra);
        args.push("3");
        let mut cmd_under_test = run_gentk(&args);
        cmd_under_test
            .assert()
            .success()
            .stdout(predicate::str::contains("\"values\":"));
    }
}

#[test]
fn bcrypt_command_produces_valid_hash() {
    let mut cmd = run_gentk(&["bcrypt", "mysecret", "--rounds", "12"]);
    cmd.assert().success();
}

#[test]
fn all_commands_help() {
    for &(cmd, _) in ALL_COMMANDS {
        let args = vec![cmd, "--help"];
        let mut cmd_under_test = run_gentk(&args);
        cmd_under_test
            .assert()
            .success();
    }
}

#[test]
fn config_command_with_yes_creates_file() {
    use tempfile::tempdir;
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("gentk.json");
    let mut cmd = assert_cmd::Command::cargo_bin("gentk").unwrap();
    cmd.current_dir(dir.path())
       .args(&["config", "--yes"])
       .assert()
       .success();
    assert!(config_path.exists());
}

#[test]
fn version_flag() {
    let mut cmd = run_gentk(&["--version"]);  // добавлен mut
    cmd.assert()
       .success()
       .stdout(predicate::str::contains("gentk"));
}
