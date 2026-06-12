// src/commands/bcrypt.rs
use crate::commands::Command;
use crate::errors::AppError;
use crate::output::printer::Printer;

pub struct BcryptCommand {
    value: String,
    rounds: u32,
    json: bool,
}

impl BcryptCommand {
    pub fn new(value: String, rounds: u32, json: bool) -> Self {
        Self { value, rounds, json }
    }
}

impl Command for BcryptCommand {
    fn execute(&self) -> Result<(), AppError> {
        let cost = self.rounds.clamp(4, 31);
        let hashed = bcrypt::hash(&self.value, cost)
            .map_err(|e| AppError::GeneratorError(format!("bcrypt error: {}", e)))?;
        let values = vec![hashed];
        Printer::new(self.json).print_values(values)?;
        Ok(())
    }
}
