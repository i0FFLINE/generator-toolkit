// src/cli/help.rs
pub fn print_help() {
    println!(
        r#"gentk - Generator Toolkit

Usage:
    gentk [options] [count]
    gentk <command> [options] [count]

Commands:
    password   Generate passwords (default)
    config     Generate configuration file
    uuid4      Generate UUIDv4
    uuid5      Generate UUIDv5 (needs --namespace and --name)
    uuid7      Generate UUIDv7
    ulid       Generate ULID
    cuid2      Generate CUID2
    nanoid     Generate Nano ID (--length, --alphabet)
    tsid       Generate TSID
    bcrypt     Generate BCRYPT

Password options:
    --length <n>            Password length (8-65535, default 13)
    --lower <true|false>    Include lowercase (default true)
    --upper <true|false>    Include uppercase (default true)
    --digits <true|false>   Include digits (default false)
    --special <true|false>  Include special chars (default false)
    --repeat <0-3>          Max consecutive repeats (0=unlimited, default 0)
    --reuse <0-3>           Max total occurrences of a char (0=unlimited, default 0)
    --exclude-ambiguous     Exclude 0,O,o,1,l,I,5,S,8,B (default false)
    --strategy <str>        retry|slide|error (default retry)

UUID5 options:
    --namespace <dns|url|oid|x500|UUID>
    --name <string>

NanoID options:
    --length <n>            ID length (default 21)
    --alphabet <string>     Custom alphabet (min 2 chars)

Other options:
    --json                  Output as JSON
    -h, --help              Show this help
    --version               Show version

Positional argument:
    count                   Number of values to generate (default 1)
"#
    );
}
