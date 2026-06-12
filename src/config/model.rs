// src/config/model.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordConfig {
    #[serde(default = "default_length")]
    pub length: u16,
    #[serde(default = "default_true")]
    pub lower: bool,
    #[serde(default = "default_true")]
    pub upper: bool,
    #[serde(default = "default_false")]
    pub digits: bool,
    #[serde(default = "default_false")]
    pub special: bool,
    #[serde(default = "default_false")]
    pub extra: bool,
    #[serde(default = "default_zero")]
    pub repeat: u8,
    #[serde(default = "default_zero")]
    pub reuse: u8,
    #[serde(default = "default_false")]
    pub exclude_ambiguous: bool,
    #[serde(default = "default_strategy")]
    pub strategy: String,
}

impl Default for PasswordConfig {
    fn default() -> Self {
        Self {
            length: default_length(),
            lower: default_true(),
            upper: default_true(),
            digits: default_false(),
            special: default_false(),
            extra: default_false(),
            repeat: default_zero(),
            reuse: default_zero(),
            exclude_ambiguous: default_false(),
            strategy: default_strategy(),
        }
    }
}

fn default_length() -> u16 {
    13
}
fn default_true() -> bool {
    true
}
fn default_false() -> bool {
    false
}
fn default_zero() -> u8 {
    0
}
fn default_strategy() -> String {
    "retry".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Uuid5Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Default for Uuid5Config {
    fn default() -> Self {
        Self {
            namespace: Some("dns".to_string()),
            name: Some("example.com".to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NanoidConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alphabet: Option<String>,
}

impl Default for NanoidConfig {
    fn default() -> Self {
        Self {
            length: Some(21),
            alphabet: None, // None – команда подставит стандартный алфавит
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub password: Option<PasswordConfig>,
    pub uuid4: Option<serde_json::Value>,
    pub uuid5: Option<Uuid5Config>,
    pub uuid7: Option<serde_json::Value>,
    pub ulid: Option<serde_json::Value>,
    pub cuid2: Option<serde_json::Value>,
    pub nanoid: Option<NanoidConfig>,
    pub tsid: Option<serde_json::Value>,
    pub bcrypt: Option<BcryptConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BcryptConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rounds: Option<u32>,
}

impl Default for BcryptConfig {
    fn default() -> Self {
        Self {
            value: Some("mysecret".to_string()),
            rounds: Some(12),
        }
    }
}
