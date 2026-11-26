use crate::error::ConfigError;
use crate::schema::Config;
use std::fs;
use std::path::Path;

pub struct ConfigLoader;

impl ConfigLoader {
    pub fn new() -> Result<Self, ConfigError> {
        Ok(Self)
    }

    pub fn load(&self, path: &Path) -> Result<Config, ConfigError> {
        if !path.exists() {
            return Err(ConfigError::FileNotFound(path.display().to_string()));
        }

        let content = fs::read_to_string(path)?;
        let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");

        match extension {
            "toml" => self.load_toml(&content),
            "json" => self.load_json(&content),
            "yaml" | "yml" => self.load_yaml(&content),
            _ => Err(ConfigError::Config(format!(
                "Unsupported file format: {}",
                extension
            ))),
        }
    }

    fn load_toml(&self, content: &str) -> Result<Config, ConfigError> {
        toml::from_str(content).map_err(ConfigError::TomlDe)
    }

    fn load_json(&self, content: &str) -> Result<Config, ConfigError> {
        serde_json::from_str(content).map_err(ConfigError::Json)
    }

    fn load_yaml(&self, content: &str) -> Result<Config, ConfigError> {
        serde_yaml::from_str(content).map_err(ConfigError::Yaml)
    }
}
