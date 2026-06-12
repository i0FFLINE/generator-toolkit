// tests/common.rs
use assert_cmd::Command;

/// Run gentk with given arguments.
/// Automatically adds `--yes` unless already present.
pub fn run_gentk(args: &[&str]) -> Command {
    let mut cmd = Command::cargo_bin("gentk").expect("gentk binary not found");
    cmd.args(args);
    if !args.contains(&"--yes") {
        cmd.arg("--yes");
    }
    cmd
}
