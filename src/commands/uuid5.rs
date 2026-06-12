// src/commands/uuid5.rs
use crate::commands::Command;
use crate::errors::AppError;
use crate::output::printer::Printer;
use uuid::Uuid;

pub struct Uuid5Command {
    namespace: Option<String>,
    name: Option<String>,
    count: u32,
    json: bool,
}

impl Uuid5Command {
    pub fn new(namespace: Option<String>, name: Option<String>, count: u32, json: bool) -> Self {
        Self {
            namespace,
            name,
            count,
            json,
        }
    }
}

impl Command for Uuid5Command {
    fn execute(&self) -> Result<(), AppError> {
        let namespace_str = self
            .namespace
            .as_ref()
            .ok_or_else(|| AppError::ValidationError("namespace is required".into()))?;
        let name = self
            .name
            .as_ref()
            .ok_or_else(|| AppError::ValidationError("name is required".into()))?;

        let namespace_uuid = match namespace_str.to_lowercase().as_str() {
            "dns" => Uuid::NAMESPACE_DNS,
            "url" => Uuid::NAMESPACE_URL,
            "oid" => Uuid::NAMESPACE_OID,
            "x500" => Uuid::NAMESPACE_X500,
            other => Uuid::parse_str(other).map_err(|_| {
                AppError::ValidationError(format!("Invalid namespace UUID: {}", other))
            })?,
        };

        let values: Vec<String> = (0..self.count)
            .map(|_| Uuid::new_v5(&namespace_uuid, name.as_bytes()).to_string())
            .collect();

        Printer::new(self.json).print_values(values)?;
        Ok(())
    }
}
