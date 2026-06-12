// src/commands/command.rs
use crate::errors::AppError;

pub trait Command {
    fn execute(&self) -> Result<(), AppError>;
}
