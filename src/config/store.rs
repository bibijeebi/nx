use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub allow_unfree: bool,
}

impl Config {
    pub fn load() -> Self {
        if let Some(config_path) = get_config_path() {
            if let Ok(contents) = fs::read_to_string(config_path) {
                if let Ok(config) = serde_json::from_str(&contents) {
                    return config;
                }
            }
        }
        Config::default()
    }

    pub fn save(&self) {
        if let Some(config_path) = get_config_path() {
            if let Some(parent) = config_path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let _ = fs::write(
                config_path,
                serde_json::to_string_pretty(self).unwrap_or_default(),
            );
        }
    }
}

fn get_config_path() -> Option<PathBuf> {
    ProjectDirs::from("com", "nx", "nx-wrapper").map(|proj_dirs| {
        proj_dirs.config_dir().to_path_buf().join("config.json")
    })
}