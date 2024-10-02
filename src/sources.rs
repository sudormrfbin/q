use anyhow::Result;
use directories::ProjectDirs;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct Sources {
    pub(crate) nicknames: HashMap<String, PathBuf>,
}

impl Default for Sources {
    fn default() -> Self {
        let hash_map = HashMap::new();
        let mut nicknames = hash_map;
        nicknames.insert("self".to_string(), path());
        Self { nicknames }
    }
}

fn path() -> PathBuf {
    let proj_dirs = ProjectDirs::from("", "e", "e").expect("Failed to get project directories");
    let config_dir = proj_dirs.config_dir();
    config_dir.join("sources.toml")
}

pub fn load() -> Result<Sources> {
    let config_path = path();
    if !config_path.exists() {
        let config = Sources::default();
        fs::create_dir_all(config_path.parent().unwrap())?;
        fs::write(&config_path, toml::to_string(&config)?)?;
        return Ok(config);
    }
    let config_str = fs::read_to_string(&config_path)?;
    Ok(toml::from_str(&config_str)?)
}
