use serde::{Serialize, Deserialize};
use std::{fs::File, io::{self, Write}, path::{Path, PathBuf}};
use toml;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub duration_secs: u64,
}

impl Config {
    pub fn new() -> io::Result<Self> {
        let config_path = Self::get_config_path();
        let path = Path::new(&config_path);
        if path.exists() {
            let config_str = std::fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&config_str).expect("Failed to parse config");
            Ok(config)
        } else {
            let config = Config { duration_secs: 15 };
            let config_toml = toml::to_string(&config).expect("Failed to serialize config");
            let mut file = File::create(&config_path)?;
            file.write_all(config_toml.as_bytes())?;
            Ok(config)
        }
    }

    fn get_config_path() -> PathBuf {
        let current_dir = std::env::current_dir().expect("Failed to determine current directory");
        current_dir.join("config.toml")
    }
}