// src/config/loader.rs
use super::model::Config;
use crate::errors::AppError;
use std::fs;
use std::path::PathBuf;

pub fn load_config(custom_path: Option<&PathBuf>) -> Result<Config, AppError> {
    // Если указан пользовательский путь, пробуем только его
    if let Some(path) = custom_path {
        let content = fs::read_to_string(path)
            .map_err(|e| AppError::ConfigError(format!("Cannot read {:?}: {}", path, e)))?;
        let config: Config = serde_json::from_str(&content)
            .map_err(|e| AppError::ConfigError(format!("Invalid JSON in {:?}: {}", path, e)))?;
        return Ok(config);
    }

    // Иначе ищем по стандартным путям
    let search_paths = get_config_paths();
    for path in search_paths {
        if path.exists() {
            let content = fs::read_to_string(&path)
                .map_err(|e| AppError::ConfigError(format!("Cannot read {:?}: {}", path, e)))?;
            if let Ok(config) = serde_json::from_str(&content) {
                return Ok(config);
            }
        }
    }
    Ok(Config::default())
}

fn get_config_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    paths.push(PathBuf::from("./gentk.json"));
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        paths.push(PathBuf::from(xdg).join("gentk/config.json"));
    } else if let Ok(home) = std::env::var("HOME") {
        paths.push(PathBuf::from(home).join(".config/gentk/config.json"));
    }
    if let Ok(appdata) = std::env::var("APPDATA") {
        paths.push(PathBuf::from(appdata).join("gentk/config.json"));
    }
    paths.push(PathBuf::from("/etc/gentk/config.json"));
    paths
}
