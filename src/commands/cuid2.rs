// src/commands/cuid2.rs
use crate::commands::Command;
use crate::errors::AppError;
use crate::output::printer::Printer;
use cuid2;

pub struct Cuid2Command {
    count: u32,
    json: bool,
}

impl Cuid2Command {
    pub fn new(count: u32, json: bool) -> Self {
        Self { count, json }
    }
}

impl Command for Cuid2Command {
    fn execute(&self) -> Result<(), AppError> {
        let values: Vec<String> = (0..self.count).map(|_| cuid2::create_id()).collect();
        Printer::new(self.json).print_values(values)?;
        Ok(())
    }
}
