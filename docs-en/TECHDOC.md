# TECHDOC.md

# gentk Technical Specification

## Overview

gentk (Generator Toolkit) is a cross-platform console application written in Rust.

Purpose:

- password generation;
- UUID generation (v4, v5, v7);
- ULID generation;
- CUID2 generation;
- Nano ID generation;
- TSID generation;
- configuration file generation.

The application must be deterministic in behavior, strictly typed, and easily extensible.

---

# Design Principles

- SOLID.
- SRP.
- No unsafe code.
- Stable Rust only.
- English language interface.
- Strict argument validation.
- Explicit error reporting.
- Small modules preferred.
- Recommended source file size: 30–130 lines.

---

# Supported Platforms

- Linux
- Windows
- macOS

---

# Dependencies

```toml
[dependencies]
clap = { version = "4.5", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "2.0"
getrandom = "0.2"
uuid = { version = "1.23", features = ["v4", "v5", "v7"] }
ulid = "1.1"
cuid2 = "0.1"
nanoid = "0.5"
tsid = "0.3"

[build-dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = "0.4"
```

No nightly features. `getrandom` provides cryptographically secure OS randomness without the need for `rand`.

---

# Commands

Commands are implemented as subcommands. Each command lives in a single file under `src/commands/`.

Supported commands:

- password (default)
- config
- uuid4
- uuid5
- uuid7
- ulid
- cuid2
- nanoid
- tsid

---

# Command Interface

Each command implements:

```rust
trait Command {
    fn execute(&self) -> Result<(), AppError>;
}
```

---

# Password Command

Default command. Invocation:

```text
gentk
gentk 10
```

The positional integer means number of generated passwords. Default count = 1.

---

# Password Options

All options have corresponding CLI arguments and config file keys.

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `--length` | u16 (8..=65535) | 13 | Password length |
| `--lower` | bool | true | Include lowercase a-z |
| `--upper` | bool | true | Include uppercase A-Z |
| `--digits` | bool | false | Include digits 0-9 |
| `--special` | bool | false | Include safe special chars `@#%-_=+:,./` |
| `--extra` | bool | false | Include extra special chars `!$^&*()[]{}¦;'"\`<>?~` |
| `--repeat` | u8 (0..=3) | 0 | Max consecutive identical chars (0=unlimited) |
| `--reuse` | u8 (0..=3) | 0 | Max total occurrences of a char (0=unlimited) |
| `--exclude-ambiguous` | bool | false | Exclude ambiguous chars `0Oo1lI5S8B` |
| `--strategy` | string | "retry" | Constraint handling strategy: retry, slide, error |

Boolean values only accept `true` or `false`.

---

# Password Defaults (JSON config)

```json
{
  "length": 13,
  "lower": true,
  "upper": true,
  "digits": false,
  "special": false,
  "extra": false,
  "repeat": 0,
  "reuse": 0,
  "exclude_ambiguous": false,
  "strategy": "retry"
}
```

---

# Character Sets

- Lowercase: `a-z`
- Uppercase: `A-Z`
- Digits: `0-9`
- Safe special: `@#%-_=+:,./`
- Extra special: `!$^&*()[]{}|;'"\`<>?~`

The charset is built at runtime based on enabled flags. If none enabled → validation error.

---

# Constraints

## repeat
Maximum consecutive identical characters. 0 means unlimited. Allowed: 0-3.

## reuse
Maximum occurrences of the same character in the password. 0 means unlimited. Allowed: 0-3.

## Strategy
- `retry` – regenerate from scratch until constraints are met (max 1000 attempts).
- `slide` – correct violating characters on the fly (non‑deterministic, uses random replacement).
- `error` – abort with error if constraints not met immediately.

Impossible constraints (e.g., `--length=8 --reuse=1 --lower=true` only) produce validation error regardless of strategy.

If `repeat > length` or `reuse > length` (with positive limits), a warning is printed via `Notice::warn`, generation continues.

---

# Random Source

Cryptographically secure randomness from the operating system is provided by the `getrandom` crate. A custom lightweight `OsRng` wrapper is defined in `src/random/os_rng.rs` and used throughout all generators. It implements methods `gen_range(usize) -> usize` and `choose(&[T]) -> Option<&T>`.

The `rand` crate is **not** used.

---

# UUID Commands

- `uuid4` – random UUID v4.
- `uuid5` – namespace-based UUID v5. Requires `--namespace` and `--name`. Default values from config (if present) are `"dns"` and `"example.com"`. Namespaces supported: `dns`, `url`, `oid`, `x500`, or a UUID string.
- `uuid7` – time-ordered UUID v7.

Count argument sets the number of UUIDs to generate.

---

# ULID, CUID2, TSID

Simple generators using the respective crates (`ulid`, `cuid2`, `tsid`). No configurable parameters; only count is accepted.

---

# Nano ID

Options:

- `--length` (default 21) – length of generated ID.
- `--alphabet` (default 62-char alphanumeric) – custom alphabet, min 2 chars.

If alphabet is not supplied, the default is used. Config file may specify both.

---

# JSON Output

Global flag `--json` for all commands.

- Single value: `{"value": "..."}`
- Multiple values: `{"values": ["...", "..."]}`

No empty lines, no extra whitespace.

---

# Configuration

Command:

```text
gentk config
```

Accepts no arguments. Writes `gentk.json` with default values for all sections: `password`, `uuid5`, `nanoid` (with concrete defaults), and empty objects for `uuid4`, `uuid7`, `ulid`, `cuid2`, `tsid`.

Priority (all commands):

1. Built-in defaults
2. `./gentk.json`
3. `$XDG_CONFIG_HOME/gentk/config.json` or `~/.config/gentk/config.json`
4. `%APPDATA%\gentk\config.json` (Windows)
5. `/etc/gentk/config.json`
6. CLI arguments (highest priority)

CLI always wins.

---

# Output Format

- Plain text: each value on its own line, with an empty line before the first and after the last value.
- With `--json`: JSON object as described.

---

# Notices

`Notice::warn` prints warnings to stderr without interrupting execution (e.g., when repeat > length).

`Notice::show` prompts for confirmation before overwriting a file, with default answer `No`.

---

# Error Handling

Single `AppError` enum (using `thiserror`) with variants:

- `CliError`
- `ConfigError`
- `ValidationError`
- `GeneratorError`
- `IoError`
- `SerializationError`
- `NoticeAborted`

---

# Versioning

Automated versioning via `build.rs` and `src/version_history.json`.

- `cargo build --release` computes a hash of `Cargo.lock`, compares with last version entry.
    - If dependencies changed → minor increment, patch reset to 0.
    - Otherwise → patch increment.
- Version is recorded in `version_history.json` with `built_successfully: false`.
- Debug builds read the latest version from the history and append `-dev`, passed to the binary via `GENTK_VERSION` environment variable.
- `gentk --version` prints the version (release: `X.Y.Z`, debug: `X.Y.Z-dev`).

---

# Source Tree (actual)

```text
src/
    main.rs
    cli/
        mod.rs
        parser.rs
        help.rs
    commands/
        mod.rs
        command.rs
        factory.rs
        password.rs
        config.rs
        uuid4.rs
        uuid5.rs
        uuid7.rs
        ulid.rs
        cuid2.rs
        nanoid.rs
        tsid.rs
    config/
        mod.rs
        model.rs
        loader.rs
        merger.rs
        validator.rs
    output/
        mod.rs
        printer.rs
        notice.rs
        confirm.rs
    errors/
        mod.rs
        app_error.rs
    random/
        mod.rs
        os_rng.rs
```

Generators are integrated into command files (no separate `generators/` directory). This keeps the architecture flat and reduces the number of small files.

---

# Help System

Default help: `gentk --help` shows password options and lists other commands.  
Command-specific help: `gentk <command> --help`.  
All help pages display allowed option ranges (e.g., `--repeat <0-3>`, `--length <8-65535>`).

---

# Future Extensions

Architecture allows adding new generators (KSUID, XID, Hashids, etc.) by creating new command files and extending the CLI parser and factory, without modifying existing commands.

TCP server mode planned for future releases.

---

# License

MIT
