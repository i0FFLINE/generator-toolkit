# TESTING.md

# Testing Guide

This document describes the testing infrastructure for `gentk`.

## Test Organisation

- `tests/common.rs` – helper module providing the `run_gentk` function that launches the binary with automatic `--yes` flag.
- `tests/integration.rs` – integration tests covering all generator commands, JSON output, help pages, configuration generation, and version flag.
- `tests/fixtures/full_config.json` – a sample configuration file used for manual testing (not yet used in automated tests).

## Running Tests

```bash
cargo test
```

All tests are integration tests executed by `cargo test`. There are no unit tests at the moment.

## Test Structure

### `tests/common.rs`

Contains one public function:

- `run_gentk(args: &[&str]) -> assert_cmd::Command`  
  Returns an `assert_cmd::Command` instance configured to run the `gentk` binary with the given arguments. If `--yes` is not already present in the arguments, it is automatically added to avoid blocking on user input prompts.

### `tests/integration.rs`

Defines the constant `ALL_COMMANDS` — an array of tuples `(subcommand_name, extra_args)` for every command that can be tested without mandatory arguments (e.g., `uuid5` requires `--namespace` and `--name` which are provided in `extra_args`).

Six test functions:

- `all_commands_generate_one_value` – verifies that each command with a count of 1 exits successfully and produces non‑empty output.
- `all_commands_json_single` – checks that `--json` flag with a single value produces a JSON containing `"value":`.
- `all_commands_json_multiple` – checks that `--json` flag with multiple values produces a JSON containing `"values":`.
- `all_commands_help` – runs `--help` for every command and asserts success.
- `config_command_with_yes_creates_file` – creates a temporary directory, runs `config --yes` inside it, and asserts that `gentk.json` is created.
- `version_flag` – runs `--version` and checks that the output contains `"gentk"`.

## Adding a New Generator

To extend the test suite for a new command:

1. Add the command implementation in `src/commands/`.
2. Add a variant to the `Commands` enum in `src/cli/parser.rs` and update `CommandFactory`.
3. In `tests/integration.rs`, add a tuple to the `ALL_COMMANDS` array:
    - If the command requires no extra arguments (e.g., `ulid`), use `("ulid", &[])`.
    - If it requires arguments (e.g., `uuid5`), pass them as `&["--namespace=dns", "--name=test"]`.

All generic tests (help, JSON, generation) will automatically run for the new command. No additional test code is needed.

## Test Dependencies

- `assert_cmd` – for spawning and asserting the binary.
- `predicates` – for flexible output assertions.
- `tempfile` – for temporary directories in isolated tests.

These are declared in `[dev-dependencies]` of `Cargo.toml`.

## Headless Mode

The `--yes` global flag is used to skip confirmation prompts (e.g., overwriting a configuration file). The `run_gentk` helper automatically appends `--yes` unless it is already present. This allows all tests to run without user interaction.

## Future Improvements

- Add unit tests for configuration merging, validation, and password constraint logic.
- Add golden tests for help output to detect unintended changes.
- Incorporate the fixture configuration file into integration tests for advanced scenarios.
