# README.md

# gentk

Generator Toolkit

Cross-platform console toolkit for generating:

- passwords;
- UUIDs;
- ULIDs;
- CUID2 identifiers;
- Nano IDs;
- TSIDs.

Written in Rust.

---

## Features

### Password generation

- cryptographically secure random source (via OS randomness);
- configurable length;
- lowercase letters;
- uppercase letters;
- digits;
- safe special characters;
- extra special characters (for advanced usage);
- consecutive repeat limits (0‑3, 0 = unlimited);
- character reuse limits (0‑3, 0 = unlimited);
- ambiguous character exclusion;
- constraint handling strategy (retry/slide/error);
- JSON configuration support.

---

### UUID generation

Supported versions:

- UUID4
- UUID5
- UUID7

UUID5 requires a namespace and name; defaults can be stored in configuration.

---

### Additional identifiers

Supported generators:

- ULID
- CUID2
- Nano ID (custom length and alphabet)
- TSID

Snowflake IDs are intentionally not supported.

---

## Supported Platforms

- Linux
- Windows
- macOS

---

## Design Goals

- predictable behavior;
- strict argument validation;
- SOLID principles;
- SRP-oriented architecture;
- small source files;
- stable Rust;
- no unsafe code;
- extensibility.

---

## Installation

### From source

```bash
cargo build --release
```

Executable:

```text
target/release/gentk
```

---

## Quick Start

Generate one password (default):

```bash
gentk
```

Generate ten passwords:

```bash
gentk 10
```

Generate twenty passwords of length 32:

```bash
gentk --length=32 20
```

---

## Password Options

Length (8‑65535, default 13):

```bash
--length=32
```

Character groups:

- `--lower=true|false` (default true) – lowercase letters a‑z
- `--upper=true|false` (default true) – uppercase letters A‑Z
- `--digits=true|false` (default false) – digits 0‑9
- `--special=true|false` (default false) – safe special characters `@#%-_=+:,./`
- `--extra=true|false` (default false) – extra special characters `!$^&*()[]{}|;'"\`<>?~`

Repeat limit (0‑3, 0 = unlimited):

```bash
--repeat=2
```

Reuse limit (0‑3, 0 = unlimited):

```bash
--reuse=3
```

Constraint strategy:

```bash
--strategy=retry   # retry (default), slide, error
```

Exclude ambiguous characters (0Oo1lI5S8B):

```bash
--exclude-ambiguous=true
```

JSON output (single value → `{"value":"..."}`, multiple → `{"values":["..."]}`):

```bash
--json
```

Generate ten passwords with all groups and extra symbols:

```bash
gentk --lower=true --upper=true --digits=true --special=true --extra=true 10
```

---

## UUID4

```bash
gentk uuid4
gentk uuid4 100 --json
```

---

## UUID5

Requires namespace and name. They can be supplied via CLI or configuration file (default namespace `dns`, default name `example.com` if config present).

```bash
gentk uuid5 --namespace=dns --name=example.com
gentk uuid5 --namespace=url --name=myapp 50
```

Supported namespaces: `dns`, `url`, `oid`, `x500`, or a UUID string.

---

## UUID7

```bash
gentk uuid7
gentk uuid7 50 --json
```

---

## ULID

```bash
gentk ulid
gentk ulid 100
```

---

## CUID2

```bash
gentk cuid2
gentk cuid2 100
```

---

## Nano ID

Default length 21, default alphabet (62 chars).

```bash
gentk nanoid
```

Custom length:

```bash
gentk nanoid --length=32
```

Custom alphabet (min 2 characters):

```bash
gentk nanoid --alphabet=abcdef123456
```

Generate fifty identifiers:

```bash
gentk nanoid 50
```

---

## TSID

```bash
gentk tsid
gentk tsid 100
```

---

## JSON Output

Add `--json` to any command. For a single value: `{"value":"..."}`, for multiple: `{"values":["...", ...]}`. No empty lines.

---

## Configuration

Generate default configuration file (writes `gentk.json` in current directory):

```bash
gentk config
```

Produces (excerpt):

```json
{
  "password": {
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
  },
  "uuid5": {
    "namespace": "dns",
    "name": "example.com"
  },
  "nanoid": {
    "length": 21,
    "alphabet": null
  },
  ...
}
```

## Configuration Search Order

1. Built-in defaults
2. `./gentk.json`
3. `$XDG_CONFIG_HOME/gentk/config.json` (or `~/.config/gentk/config.json`)
4. `%APPDATA%\gentk\config.json` (Windows)
5. `/etc/gentk/config.json`
6. Command-line arguments (highest priority)

---

## Help System

Main help:

```bash
gentk --help
```

Command-specific help:

```bash
gentk --help config
gentk --help uuid4
gentk --help uuid5
...
```

All help pages show allowed ranges (e.g., `--repeat <0-3>`, `--length <8-65535>`).

---

## Version

```bash
gentk --version
```

Version is automatically derived from build history. Release builds receive a full semver; debug builds append `-dev`.

---

## Project Structure

```text
src/
    main.rs
    cli/
    commands/
        password.rs, config.rs, uuid4.rs, uuid5.rs, uuid7.rs,
        ulid.rs, cuid2.rs, nanoid.rs, tsid.rs
    config/
    errors/
    output/
    random/
```

---

## Documentation

- `docs/USERDOC.md` – user guide
- `docs/TECHDOC.md` – technical specification
- `docs/ARCHITECTURE.md` – architecture description
- Russian versions available (`*-ru.md`)

---

## License

MIT
