// src/commands/password.rs
use crate::commands::Command;
use crate::errors::AppError;
use crate::output::notice::Notice;
use crate::output::printer::Printer;
use crate::random::OsRng;

pub struct PasswordCommand {
    length: u16,
    lower: bool,
    upper: bool,
    digits: bool,
    special: bool,
    extra: bool,
    repeat: u8,
    reuse: u8,
    exclude_ambiguous: bool,
    strategy: String,
    count: u32,
    json: bool,
}

impl PasswordCommand {
    pub fn new(
        length: u16,
        lower: bool,
        upper: bool,
        digits: bool,
        special: bool,
        extra: bool,
        repeat: u8,
        reuse: u8,
        exclude_ambiguous: bool,
        strategy: String,
        count: u32,
        json: bool,
    ) -> Self {
        Self {
            length,
            lower,
            upper,
            digits,
            special,
            extra,
            repeat,
            reuse,
            exclude_ambiguous,
            strategy,
            count,
            json,
        }
    }

    fn build_charset(&self) -> Vec<char> {
        let mut charset = Vec::new();
        if self.lower {
            charset.extend('a'..='z');
        }
        if self.upper {
            charset.extend('A'..='Z');
        }
        if self.digits {
            charset.extend('0'..='9');
        }
        if self.special {
            charset.extend("@#%-_=+:,./".chars());
        }
        if self.extra {
            charset.extend("!$^&*()[]{}|;'\"`<>?~".chars());
            charset.push('\\');
        }
        if self.exclude_ambiguous {
            charset.retain(|c| {
                !matches!(c, '0' | 'O' | 'o' | '1' | 'l' | 'I' | '5' | 'S' | '8' | 'B')
            });
        }
        charset
    }

    fn is_feasible(&self, charset_len: usize) -> Result<(), AppError> {
        if self.reuse > 0 && (self.length as usize) > charset_len * (self.reuse as usize) {
            return Err(AppError::ValidationError(format!(
                "Impossible constraints: length={}, reuse={}, charset size={}",
                self.length, self.reuse, charset_len
            )));
        }
        if self.repeat > 0 && self.repeat as u16 > self.length {
            Notice::warn(&format!(
                "repeat limit {} > length {}, ignoring repeat",
                self.repeat, self.length
            ));
        }
        if self.reuse > 0 && self.reuse as u16 > self.length {
            Notice::warn(&format!(
                "reuse limit {} > length {}, ignoring reuse",
                self.reuse, self.length
            ));
        }
        Ok(())
    }

    fn generate_retry(&self, charset: &[char]) -> Result<String, AppError> {
        for _ in 0..1000 {
            let candidate = self.generate_random(charset);
            if self.check_constraints(&candidate) {
                return Ok(candidate);
            }
        }
        Err(AppError::GeneratorError(
            "Failed to generate password after 1000 attempts".into(),
        ))
    }

    fn generate_slide(&self, charset: &[char]) -> Result<String, AppError> {
        let mut rng = OsRng::new();
        let mut password = self.generate_random(charset);
        if self.repeat > 0 {
            let mut chars: Vec<char> = password.chars().collect();
            let mut i = 0;
            while i + (self.repeat as usize) < chars.len() {
                let window = &chars[i..i + (self.repeat as usize) + 1];
                if window.windows(2).all(|w| w[0] == w[1]) {
                    chars[i + self.repeat as usize] = *rng.choose(charset).unwrap();
                    i = 0;
                } else {
                    i += 1;
                }
            }
            password = chars.into_iter().collect();
        }
        Ok(password)
    }

    fn generate_error(&self, _charset: &[char]) -> Result<String, AppError> {
        Err(AppError::GeneratorError(
            "Constraints cannot be satisfied with error strategy".into(),
        ))
    }

    fn generate_random(&self, charset: &[char]) -> String {
        let mut rng = OsRng::new();
        (0..self.length)
            .map(|_| *rng.choose(charset).unwrap())
            .collect()
    }

    fn check_constraints(&self, s: &str) -> bool {
        if self.repeat > 0 {
            let mut consecutive = 1;
            let chars: Vec<char> = s.chars().collect();
            for i in 1..chars.len() {
                if chars[i] == chars[i - 1] {
                    consecutive += 1;
                    if consecutive > self.repeat as usize {
                        return false;
                    }
                } else {
                    consecutive = 1;
                }
            }
        }
        if self.reuse > 0 {
            let mut counts = std::collections::HashMap::new();
            for c in s.chars() {
                let cnt = counts.entry(c).or_insert(0);
                *cnt += 1;
                if *cnt > self.reuse as usize {
                    return false;
                }
            }
        }
        true
    }
}

impl Command for PasswordCommand {
    fn execute(&self) -> Result<(), AppError> {
        let charset = self.build_charset();
        if charset.is_empty() {
            return Err(AppError::ValidationError(
                "No character groups selected".into(),
            ));
        }
        self.is_feasible(charset.len())?;

        let mut passwords = Vec::with_capacity(self.count as usize);
        for _ in 0..self.count {
            let pwd = match self.strategy.as_str() {
                "retry" => self.generate_retry(&charset),
                "slide" => self.generate_slide(&charset),
                "error" => self.generate_error(&charset),
                _ => Err(AppError::ValidationError("Invalid strategy".into())),
            }?;
            passwords.push(pwd);
        }
        Printer::new(self.json).print_values(passwords)?;
        Ok(())
    }
}
