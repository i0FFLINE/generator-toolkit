# ARCHITECTURE.md

# gentk Architecture

## Purpose

This document describes the internal architecture of gentk.

The primary goals are:

- maintainability;
- extensibility;
- predictability;
- strict separation of responsibilities;
- low coupling;
- high cohesion.

The architecture intentionally favors explicit code over clever abstractions.

---

# Design Principles

## SOLID

The project follows SOLID principles.

### Single Responsibility Principle

Every module should have one reason to change.

Examples:

- CLI parsing;
- configuration loading;
- UUID generation;
- NanoID generation;
- output formatting.

must live in different modules.

---

### Open/Closed Principle

New generators should be added without modifying existing implementations.

Adding:

- KSUID;
- XID;
- Hashids;

should require adding new modules rather than changing existing ones.

---

### Liskov Substitution Principle

Every command implementation must satisfy the Command contract.

---

### Interface Segregation Principle

Traits should remain small.

Avoid large "god traits".

Preferred:

```rust
trait Command {
    fn execute(&self) -> Result<(), AppError>;
}
```

---

### Dependency Inversion Principle

High-level modules should depend on abstractions.

Generators should not depend directly on CLI internals.

---

# Layered Architecture

The application consists of several layers.

```text
CLI
 ↓
Command Factory
 ↓
Command
 ↓
Output
```

Within commands, the generation logic is embedded directly in the command file (no separate `generators/` layer). This keeps the structure flat and avoids excessive file fragmentation for simple wrappers.

Configuration is available to every layer that needs it.

---

# Layers

## CLI Layer

Responsible for:

- parsing arguments;
- type validation;
- help pages;
- subcommands.

Must not:

- generate values;
- perform business logic.

Directory:

```text
src/cli
```

Help system is implemented via `clap` derive macros with `help` attributes. Allowed ranges (e.g., `--length <8-65535>`) are enforced by `value_parser!` and displayed automatically.

---

## Command Layer

Responsible for:

- command execution;
- orchestration;
- encapsulating generator logic for that specific command.

Must not:

- know argument parsing details;
- know serialization details.

Directory:

```text
src/commands
```

Each command (password, uuid4, uuid5, etc.) is a single file that contains both the command struct and its generation logic. This keeps the generator and its command tightly cohesive without introducing a separate `generators/` directory for trivial cases.

---

## Configuration Layer

Responsible for:

- defaults;
- loading configuration files;
- merging priorities;
- validation.

Directory:

```text
src/config
```

### Configuration Priority

All commands share the same configuration file `gentk.json` with per-command sections.

Search order:

```text
built-in defaults
    ↓
./gentk.json
    ↓
$XDG_CONFIG_HOME/gentk/config.json
    ↓
%APPDATA%\gentk\config.json
    ↓
/etc/gentk/config.json
    ↓
CLI arguments
```

CLI arguments always win.

Config command generates a complete `gentk.json` with concrete default values for `password`, `uuid5`, and `nanoid` sections, and empty objects for others.

---

## Output Layer

Responsible for:

- console printing;
- notices;
- confirmations.

Directory:

```text
src/output
```

- `Printer` handles plain text (empty lines around values) and JSON output (`{"value": ...}` or `{"values": [...]}`).
- `Notice::warn` prints warnings to stderr without stopping execution.
- `Notice::show` prompts for confirmation with default answer `No`.

---

## Error Layer

Responsible for:

- application errors.

Directory:

```text
src/errors
```

Single `AppError` enum using `thiserror`. Variants:

- `CliError`
- `ConfigError`
- `ValidationError`
- `GeneratorError`
- `IoError`
- `SerializationError`
- `NoticeAborted`

---

## Randomness Layer

Responsible for:

- cryptographically secure random number generation.

Directory:

```text
src/random
```

Uses `getrandom` crate (direct OS randomness). A lightweight `OsRng` wrapper provides `gen_range(usize) -> usize` and `choose(&[T]) -> Option<&T>`. The `rand` crate is not used.

All generators obtain randomness through this `OsRng` (or its methods). No custom RNG implementations are allowed.

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

Generators live inside their respective command files. No separate `generators/` directory exists.

---

# File Size Policy

Preferred: 30–100 lines.
Acceptable: 30–130 lines.
Avoid files larger than 150 lines; they should be split.

---

# Commands

Every command implements:

```rust
trait Command {
    fn execute(&self) -> Result<(), AppError>;
}
```

---

# Command Factory

Responsible for:

- constructing the appropriate command based on CLI args and configuration.
- merging defaults from config with CLI overrides.

Its only responsibility is construction, not execution.

---

# Generators

Generation logic is embedded in the command file (e.g., `password.rs` contains password generation, `nanoid.rs` contains Nano ID generation). This keeps the command and its generator cohesive and reduces the number of files.

Generators must not print output; they return values. Output formatting is handled by the `Printer`.

---

# Data Flow

```text
CLI
 ↓
Typed command arguments (+ config)
 ↓
Factory
 ↓
Command (with built‑in generator)
 ↓
Output (Printer)
```

---

# Dependency Rules

Allowed:

```text
CLI → Command → Output
Command → Random
Command → Config (via factory)
```

Forbidden:

- Commands must not depend on CLI internals.
- Generators must not print directly.
- Generators must not load configuration themselves.

---

# Notices

Notice flow for confirmation:

```text
detect issue
    ↓
show explanation
    ↓
ask confirmation (Continue? [y/N])
    ↓
continue or abort
```

`Notice::warn` issues a warning but continues execution. Implicit conflicts (e.g., `repeat > length`) produce a warning, and generation proceeds.

---

# Help System

Every command has its own help page, automatically generated by `clap`. Help shows allowed ranges and possible values.

```text
gentk --help
gentk password --help
gentk uuid5 --help
...
```

---

# Versioning (Automated)

- `build.rs` computes a hash of `Cargo.lock` on `--release` builds.
- Version history stored in `src/version_history.json`.
- On dependency change → minor version increment, patch reset.
- On same dependencies → patch increment.
- Debug builds read the latest version and append `-dev`, passed via `GENTK_VERSION` env var.
- `--version` flag prints the version.

---

# Future Extensions

New generators are added by:

1. Creating a new command file in `src/commands/`.
2. Adding a variant to the `Commands` enum in `cli/parser.rs`.
3. Extending `CommandFactory::create` to handle the new variant.

Existing commands remain untouched. TCP server mode is planned.

---

# Testing

Tests are not yet implemented but will include:

- unit tests for validators, merger, generators, parser;
- integration tests for CLI commands, notices, config generation;
- golden tests for help output and JSON files.

---

# License

MIT
