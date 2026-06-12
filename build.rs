use std::env;
use std::fs;
use std::path::Path;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
struct VersionEntry {
    version: String,
    deps_hash: String,
    built_successfully: bool,
    timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct VersionHistory {
    versions: Vec<VersionEntry>,
}

fn main() {
    let profile = env::var("PROFILE").unwrap_or_default();
    let history_path = Path::new("src/version_history.json");

    if profile != "release" {
        // Debug: просто читаем последнюю версию и передаём с -dev
        let dev_version = if let Ok(data) = fs::read_to_string(history_path) {
            if let Ok(history) = serde_json::from_str::<VersionHistory>(&data) {
                history.versions.last()
                       .map(|e| format!("{}-dev", e.version))
                       .unwrap_or_else(|| "0.0.0-dev".to_string())
            } else {
                "0.0.0-dev".to_string()
            }
        } else {
            "0.0.0-dev".to_string()
        };

        println!("cargo:rustc-env=GENTK_VERSION={}", dev_version);
        println!("cargo:rerun-if-changed=build.rs");
        println!("cargo:rerun-if-changed=src/version_history.json");
        return;
    }

    // Release: полный цикл версионирования
    let mut history: VersionHistory = if history_path.exists() {
        let data = fs::read_to_string(history_path).expect("Failed to read version_history.json");
        serde_json::from_str(&data).unwrap_or(VersionHistory { versions: vec![] })
    } else {
        VersionHistory { versions: vec![] }
    };

    let deps_hash = compute_deps_hash();

    let new_version = if let Some(last) = history.versions.last() {
        if last.deps_hash != deps_hash {
            let mut parts: Vec<u32> = last.version.split('.')
                                          .filter_map(|s| s.parse().ok())
                                          .collect();
            if parts.len() >= 2 {
                parts[1] += 1;
                parts[2] = 0;
                format!("{}.{}.{}", parts[0], parts[1], parts[2])
            } else {
                "0.1.0".to_string()
            }
        } else {
            let mut parts: Vec<u32> = last.version.split('.')
                                          .filter_map(|s| s.parse().ok())
                                          .collect();
            if parts.len() >= 3 {
                parts[2] += 1;
                format!("{}.{}.{}", parts[0], parts[1], parts[2])
            } else {
                "0.0.1".to_string()
            }
        }
    } else {
        "0.1.0".to_string()
    };

    let entry = VersionEntry {
        version: new_version.clone(),
        deps_hash,
        built_successfully: false,
        timestamp: Utc::now().to_rfc3339(),
    };

    history.versions.push(entry);
    let json = serde_json::to_string_pretty(&history).expect("Failed to serialize history");
    fs::write(history_path, json).expect("Failed to write version_history.json");

    println!("cargo:rustc-env=GENTK_VERSION={}", new_version);
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.lock");
    println!("cargo:rerun-if-changed=src/version_history.json");
}

fn compute_deps_hash() -> String {
    let lock_path = "Cargo.lock";
    if Path::new(lock_path).exists() {
        let data = fs::read_to_string(lock_path).unwrap_or_default();
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    } else {
        "no_lock".to_string()
    }
}
