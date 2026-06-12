// src/output/printer.rs
use serde_json::json;
use std::io::{self, Write};

pub struct Printer {
    json_mode: bool,
}

impl Printer {
    pub fn new(json_mode: bool) -> Self {
        Self { json_mode }
    }

    pub fn print_values(&self, values: Vec<String>) -> Result<(), io::Error> {
        let stdout = io::stdout();
        let mut handle = stdout.lock();

        if self.json_mode {
            let output = if values.len() == 1 {
                json!({ "value": values[0] })
            } else {
                json!({ "values": values })
            };
            writeln!(handle, "{}", serde_json::to_string(&output)?)?;
        } else {
            if !values.is_empty() {
                writeln!(handle)?;
                for v in values {
                    writeln!(handle, "{}", v)?;
                }
                writeln!(handle)?;
            }
        }
        Ok(())
    }
}
