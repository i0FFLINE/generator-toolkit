// src/output/notice.rs
use crate::errors::AppError;

pub struct Notice;

impl Notice {
    /// Show a notice and ask for confirmation.
    /// If `force_yes` is true, auto-confirm without waiting for input.
    pub fn show(message: &str, force_yes: bool) -> Result<(), AppError> {
        if force_yes {
            eprintln!("Notice: {} (auto-confirmed)", message);
            return Ok(());
        }
        eprintln!("Notice:\n{}\nContinue? [y/N]", message);
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .map_err(|e| AppError::IoError(e))?;
        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => Ok(()),
            _ => Err(AppError::NoticeAborted),
        }
    }

    pub fn warn(message: &str) {
        eprintln!("Warning: {}", message);
    }
}
