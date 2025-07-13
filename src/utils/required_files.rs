use crate::models::config::Config;
use crate::utils::constants::*;
use crate::utils::logging::*;
use serde_json;
use std::io::Write;
use std::path::PathBuf;

// base
pub fn localyze_dir() -> PathBuf {
    PathBuf::from(format!(".{}", APPNAME))
}

pub fn ensure_localyze_dir_exists() -> std::io::Result<()> {
    let localyze_dir = localyze_dir();
    if !localyze_dir.exists() {
        std::fs::create_dir_all(&localyze_dir)?;
    }
    Ok(())
}

// credentials
pub fn credentials_path() -> PathBuf {
    localyze_dir().join("credentials.json")
}

pub fn credentials_exists() -> bool {
    credentials_path().exists()
}

// config
pub fn config_path() -> PathBuf {
    localyze_dir().join("config.json")
}

pub fn config_exists() -> bool {
    config_path().exists()
}

pub fn create_config_file(contents: &str) -> std::io::Result<()> {
    std::fs::write(config_path(), contents)?;
    Ok(())
}

pub fn open_config_file() -> Config {
    let path = localyze_dir().join("config.json");
    let contents = std::fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!("Failed to read config file at {:?}: {}", path, e);
    });
    serde_json::from_str::<Config>(&contents).unwrap_or_else(|e| {
        panic!("Failed to parse config file at {:?}: {}", path, e);
    })
}

// .gitignore
pub fn ensure_gitignore_has_credentials_path() -> std::io::Result<()> {
    let path = ".gitignore";
    let entry = format!(".{}/credentials.json", APPNAME);

    let needs_write = match std::fs::read_to_string(path) {
        Ok(contents) => !contents.lines().any(|l| l.trim() == entry),
        Err(_) => true,
    };

    if needs_write {
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;

        writeln!(file, "{}", entry)?;
        log_info(&format!("Added {} to .gitignore", entry));
    }

    Ok(())
}
