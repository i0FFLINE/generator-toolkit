// src/commands/nanoid.rs
use crate::commands::Command;
use crate::errors::AppError;
use crate::output::printer::Printer;
use crate::random::OsRng;

const DEFAULT_ALPHABET: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub struct NanoidCommand {
    length: usize,
    alphabet: String,
    count: u32,
    json: bool,
}

impl NanoidCommand {
    pub fn new(length: u16, alphabet: Option<String>, count: u32, json: bool) -> Self {
        let alphabet = alphabet.unwrap_or_else(|| DEFAULT_ALPHABET.to_string());
        Self {
            length: length as usize,
            alphabet,
            count,
            json,
        }
    }
}

impl Command for NanoidCommand {
    fn execute(&self) -> Result<(), AppError> {
        let chars: Vec<char> = self.alphabet.chars().collect();
        if chars.len() < 2 {
            return Err(AppError::ValidationError(
                "Alphabet must be at least 2 characters".into(),
            ));
        }
        let mut rng = OsRng::new();
        let values: Vec<String> = (0..self.count)
            .map(|_| {
                (0..self.length)
                    .map(|_| chars[rng.gen_range(chars.len())])
                    .collect()
            })
            .collect();
        Printer::new(self.json).print_values(values)?;
        Ok(())
    }
}
