// src/cli/parser.rs
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "gentk")]
#[command(about = "Generator Toolkit — passwords, UUIDs, ULIDs, Nano IDs and more")]
pub struct CliArgs {
    #[arg(long, global = true, help = "Print version")]
    pub version: bool,

    #[arg(long, global = true, help = "Path to a custom configuration file")]
    pub config_file: Option<PathBuf>,

    #[arg(global = true, help = "Number of values to generate (default: 1)")]
    pub count: Option<u32>,

    #[arg(long, global = true, help = "Output in JSON format")]
    pub json: bool,

    #[arg(long, global = true, help = "Skip all confirmations (assume yes)")]
    pub yes: bool,

    // Password options (global)
    #[arg(long, require_equals = true, num_args = 1,
        value_parser = clap::value_parser!(bool),
        help = "Include lowercase letters [default: true]")]
    pub lower: Option<bool>,

    #[arg(long, require_equals = true, num_args = 1,
        value_parser = clap::value_parser!(bool),
        help = "Include uppercase letters [default: true]")]
    pub upper: Option<bool>,

    #[arg(long, require_equals = true, num_args = 1,
        value_parser = clap::value_parser!(bool),
        help = "Include digits [default: false]")]
    pub digits: Option<bool>,

    #[arg(long, require_equals = true, num_args = 1,
        value_parser = clap::value_parser!(bool),
        help = "Include safe special characters (@#%-_=+:,./) [default: false]")]
    pub special: Option<bool>,

    #[arg(long, require_equals = true, num_args = 1,
        value_parser = clap::value_parser!(bool),
        help = "Include extra special characters (!$^&*()[]{}|;'\"<>?~) [default: false]")]
    pub extra: Option<bool>,

    #[arg(long,
          value_parser = clap::value_parser!(u16).range(8..=65535),
          help = "Password length")]
    pub length: Option<u16>,

    #[arg(long,
          value_parser = clap::value_parser!(u8).range(0..=3),
          help = "Max consecutive identical characters (0 = unlimited)")]
    pub repeat: Option<u8>,

    #[arg(long,
          value_parser = clap::value_parser!(u8).range(0..=3),
          help = "Max total occurrences of a character (0 = unlimited)")]
    pub reuse: Option<u8>,

    #[arg(long, require_equals = true, num_args = 1,
        value_parser = clap::value_parser!(bool),
        help = "Exclude ambiguous characters (0Oo1lI5S8B) [default: false]")]
    pub exclude_ambiguous: Option<bool>,

    #[arg(long, require_equals = true, num_args = 1,
          value_parser = clap::builder::PossibleValuesParser::new(["retry", "slide", "error"]),
          help = "Constraint strategy: retry (default), slide, error")]
    pub strategy: Option<String>,

    // Other generators' options (global)
    #[arg(
        long,
        help = "Namespace for UUID5 (dns, url, oid, x500, or a UUID string)"
    )]
    pub namespace: Option<String>,

    #[arg(long, help = "Name for UUID5")]
    pub name: Option<String>,

    #[arg(long, help = "Custom alphabet for Nano ID (min 2 characters)")]
    pub alphabet: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate passwords
    Password {
        #[arg(long, require_equals = true, num_args = 1,
            value_parser = clap::value_parser!(bool),
            help = "Include lowercase letters")]
        lower: Option<bool>,

        #[arg(long, require_equals = true, num_args = 1,
            value_parser = clap::value_parser!(bool),
            help = "Include uppercase letters")]
        upper: Option<bool>,

        #[arg(long, require_equals = true, num_args = 1,
            value_parser = clap::value_parser!(bool),
            help = "Include digits")]
        digits: Option<bool>,

        #[arg(long, require_equals = true, num_args = 1,
            value_parser = clap::value_parser!(bool),
            help = "Include safe special characters (@#%-_=+:,./)")]
        special: Option<bool>,

        #[arg(long, require_equals = true, num_args = 1,
            value_parser = clap::value_parser!(bool),
            help = "Include extra special characters (!$^&*()[]{}|;'\"<>?~)")]
        extra: Option<bool>,

        #[arg(long,
              value_parser = clap::value_parser!(u16).range(8..=65535),
              help = "Password length")]
        length: Option<u16>,

        #[arg(long,
              value_parser = clap::value_parser!(u8).range(0..=3),
              help = "Max consecutive identical characters (0 = unlimited)")]
        repeat: Option<u8>,

        #[arg(long,
              value_parser = clap::value_parser!(u8).range(0..=3),
              help = "Max total occurrences of a character (0 = unlimited)")]
        reuse: Option<u8>,

        #[arg(long, require_equals = true, num_args = 1,
            value_parser = clap::value_parser!(bool),
            help = "Exclude ambiguous characters (0Oo1lI5S8B)")]
        exclude_ambiguous: Option<bool>,

        #[arg(long, require_equals = true, num_args = 1,
              value_parser = clap::builder::PossibleValuesParser::new(["retry", "slide", "error"]),
              help = "Constraint strategy")]
        strategy: Option<String>,

        /// Number of passwords to generate
        count: Option<u32>,
    },

    /// Generate configuration file with defaults
    Config,

    /// Generate UUID version 4
    Uuid4 {
        /// Number of UUIDs to generate
        count: Option<u32>,
    },

    /// Generate UUID version 5 (requires --namespace and --name)
    Uuid5 {
        #[arg(long, help = "Namespace (dns, url, oid, x500, or UUID)")]
        namespace: Option<String>,

        #[arg(long, help = "Name string")]
        name: Option<String>,

        /// Number of UUIDs to generate
        count: Option<u32>,
    },

    /// Generate UUID version 7
    Uuid7 {
        /// Number of UUIDs to generate
        count: Option<u32>,
    },

    /// Generate ULID
    Ulid {
        /// Number of ULIDs to generate
        count: Option<u32>,
    },

    /// Generate CUID2
    Cuid2 {
        /// Number of CUID2s to generate
        count: Option<u32>,
    },

    /// Generate Nano ID
    Nanoid {
        #[arg(long,
              value_parser = clap::value_parser!(u16).range(1..),
              help = "ID length")]
        length: Option<u16>,

        #[arg(long, help = "Custom alphabet (min 2 characters)")]
        alphabet: Option<String>,

        /// Number of Nano IDs to generate
        count: Option<u32>,
    },

    /// Generate TSID
    Tsid {
        /// Number of TSIDs to generate
        count: Option<u32>,
    },

    /// Hash a string with bcrypt (Laravel-compatible)
    Bcrypt {
        /// String to hash
        value: String,

        #[arg(long, value_parser = clap::value_parser!(u32).range(4..=31),
          help = "Number of rounds (4–31, default 12)")]
        rounds: Option<u32>,
    },
}
