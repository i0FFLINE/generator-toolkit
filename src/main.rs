// src/main.rs
mod cli;
mod commands;
mod config;
mod errors;
mod output;
mod random;

use clap::Parser;
use cli::parser::CliArgs;
use commands::CommandFactory;
use config::loader::load_config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();

    if args.version {
        println!(
            "gentk {}",
            option_env!("GENTK_VERSION").unwrap_or("0.0.0-dev")
        );
        return Ok(());
    }

    let config = load_config(args.config_file.as_ref())?;
    let command = CommandFactory::create(args, config);
    command.execute()?;
    Ok(())
}
