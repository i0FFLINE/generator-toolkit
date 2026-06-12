// src/commands/uuid7.rs
use crate::commands::Command;
use crate::errors::AppError;
use crate::output::printer::Printer;
use uuid::Uuid;

pub struct Uuid7Command {
    count: u32,
    json: bool,
}

impl Uuid7Command {
    pub fn new(count: u32, json: bool) -> Self {
        Self { count, json }
    }
}

impl Command for Uuid7Command {
    fn execute(&self) -> Result<(), AppError> {
        let values: Vec<String> = (0..self.count)
            .map(|_| Uuid::now_v7().to_string())
            .collect();
        Printer::new(self.json).print_values(values)?;
        Ok(())
    }
}
