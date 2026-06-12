// src/commands/tsid.rs
use crate::commands::Command;
use crate::errors::AppError;
use crate::output::printer::Printer;
use tsid;

pub struct TsidCommand {
    count: u32,
    json: bool,
}

impl TsidCommand {
    pub fn new(count: u32, json: bool) -> Self {
        Self { count, json }
    }
}

impl Command for TsidCommand {
    fn execute(&self) -> Result<(), AppError> {
        let values: Vec<String> = (0..self.count)
            .map(|_| tsid::create_tsid().to_string())
            .collect();
        Printer::new(self.json).print_values(values)?;
        Ok(())
    }
}
