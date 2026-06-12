// src/commands/ulid.rs
use crate::commands::Command;
use crate::errors::AppError;
use crate::output::printer::Printer;
use ulid::Ulid;

pub struct UlidCommand {
    count: u32,
    json: bool,
}

impl UlidCommand {
    pub fn new(count: u32, json: bool) -> Self {
        Self { count, json }
    }
}

impl Command for UlidCommand {
    fn execute(&self) -> Result<(), AppError> {
        let values: Vec<String> = (0..self.count).map(|_| Ulid::new().to_string()).collect();
        Printer::new(self.json).print_values(values)?;
        Ok(())
    }
}
