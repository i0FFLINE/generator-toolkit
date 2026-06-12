# USERDOC.md

# gentk User Guide

## Introduction

gentk is a console toolkit for generating:

- passwords;
- UUIDs;
- ULIDs;
- CUID2 identifiers;
- Nano IDs;
- TSIDs.

---

# Password Generation

Generate one password (default):

```text
gentk
```

Generate ten passwords:

```text
gentk 10
```

Generate twenty passwords with digits:

```text
gentk --digits=true 20
```

Generate passwords using all groups including extra symbols:

```text
gentk --lower=true --upper=true --digits=true --special=true --extra=true 20
```

---

# Password Length

Example:

```text
gentk --length=32
```

Minimum: 8, default: 13, maximum: 65535.

---

# Character Groups

Lowercase (a-z):

```text
--lower=true
```

Uppercase (A-Z):

```text
--upper=true
```

Digits (0-9):

```text
--digits=true
```

Safe special characters (@#%-_=+:,./):

```text
--special=true
```

Extra special characters (!$^&*()[]{}|;'"\`<>?~):

```text
--extra=true
```

Groups can be combined. At least one group must be enabled.

---

# Repeat Limit

Consecutive repeats (0-3, 0 = unlimited):

```text
--repeat=2
```

Example: `AA` allowed, `AAA` not allowed.

---

# Reuse Limit

Maximum total occurrences of a character (0-3, 0 = unlimited):

```text
--reuse=3
```

No character may appear more than three times.

---

# Strategy

How to satisfy constraints when generation fails:

```text
--strategy=retry   # retry (default), slide, error
```

- `retry` – regenerate up to 1000 times.
- `slide` – replace violating characters on the fly.
- `error` – abort with an error.

Use `retry` for most cases.

---

# Excluding Ambiguous Characters

```text
--exclude-ambiguous=true
```

Removes: `0 O o 1 l I 5 S 8 B`.

---

# JSON Output

Add `--json` to any command.

Single value: `{"value":"..."}`, multiple: `{"values":["...", "..."]}`.

Example:

```text
gentk --json
gentk uuid4 3 --json
```

---

# UUID4

Generate one UUID4:

```text
gentk uuid4
```

Generate one hundred:

```text
gentk uuid4 100 --json
```

---

# UUID5

Requires namespace and name. If not supplied via CLI, defaults from configuration file are used (`dns` and `example.com` if a config exists).

Example:

```text
gentk uuid5 --namespace=dns --name=example.com
```

Generate ten:

```text
gentk uuid5 --namespace=url --name=myapp 10
```

Supported namespaces: `dns`, `url`, `oid`, `x500`, or a UUID string.

---

# UUID7

Generate:

```text
gentk uuid7
```

Generate fifty:

```text
gentk uuid7 50 --json
```

---

# ULID

Generate:

```text
gentk ulid
gentk ulid 100
```

---

# CUID2

Generate:

```text
gentk cuid2
gentk cuid2 100
```

---

# Nano ID

Default length 21, default 62-character alphabet.

```text
gentk nanoid
```

Custom length:

```text
gentk nanoid --length=32
```

Custom alphabet (min 2 chars):

```text
gentk nanoid --alphabet=abcdef123
```

Generate fifty:

```text
gentk nanoid 50
```

---

# TSID

Generate:

```text
gentk tsid
gentk tsid 100
```

---

# Configuration

Create a default configuration file:

```text
gentk config
```

Writes `gentk.json` in the current directory. Example contents:

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
  }
}
```

Search order:
1. Built-in defaults
2. `./gentk.json`
3. `~/.config/gentk/config.json`
4. `/etc/gentk/config.json`
5. CLI arguments (highest priority)

---

# Help

Main help:

```text
gentk --help
```

Command-specific help:

```text
gentk --help config
gentk --help uuid5
gentk --help nanoid
...
```

All help pages show allowed ranges (e.g., `--repeat <0-3>`, `--length <8-65535>`).

---

# Version

```text
gentk --version
```

Release builds show full semver, debug builds append `-dev`.

---

# License

MIT
