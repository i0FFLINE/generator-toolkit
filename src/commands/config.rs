// src/commands/config.rs
use crate::commands::Command;
use crate::config::model::{BcryptConfig, Config, NanoidConfig, PasswordConfig, Uuid5Config};
use crate::errors::AppError;
use crate::output::notice::Notice;
use serde_json;
use std::fs;
use std::path::PathBuf;

pub struct ConfigCommand {
    json: bool,
    yes: bool,
}

impl ConfigCommand {
    pub fn new(json: bool, yes: bool) -> Self {
        Self { json, yes }
    }
}

impl Command for ConfigCommand {
    fn execute(&self) -> Result<(), AppError> {
        let config = Config {
            password: Some(PasswordConfig::default()),
            uuid5: Some(Uuid5Config::default()),
            nanoid: Some(NanoidConfig::default()),
            bcrypt: Some(BcryptConfig::default()),
            ..Default::default()
        };

        let pretty = serde_json::to_string_pretty(&config)
            .map_err(|e| AppError::SerializationError(e.to_string()))?;

        let target_path = PathBuf::from("gentk.json");

        if target_path.exists() && !self.json {
            Notice::show(
                &format!("File '{:?}' already exists. Overwrite?", target_path),
                self.yes,
            )?;
        }

        fs::write(&target_path, pretty.as_bytes()).map_err(|e| AppError::IoError(e))?;

        if !self.json {
            println!("Configuration written to {:?}", target_path);
        } else {
            let printer = crate::output::printer::Printer::new(true);
            printer.print_values(vec![pretty])?;
        }

        Ok(())
    }
}
