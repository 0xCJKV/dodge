use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub blog_title: String,
    pub theme: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            blog_title: "Dodge SSG".to_string(),
            theme: "hacker".to_string(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Path::new("config.toml");
        
        if config_path.exists() {
            let config_content = fs::read_to_string(config_path)?;
            let config: Config = toml::from_str(&config_content)?;
            Ok(config)
        } else {
            // Create default config file if it doesn't exist
            let default_config = Config::default();
            let toml_content = toml::to_string_pretty(&default_config)?;
            fs::write(config_path, toml_content)?;
            Ok(default_config)
        }
    }
}