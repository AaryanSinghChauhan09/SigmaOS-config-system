use crate::error::ConfigError;
use crate::schema::Config;
use std::path::Path;
use std::fs;

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
        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
        
        match extension {
            "toml" => self.load_toml(&content),
            "json" => self.load_json(&content),
            "yaml" | "yml" => self.load_yaml(&content),
            _ => Err(ConfigError::Config(format!("Unsupported file format: {}", extension))),
        }
    }
    
    fn load_toml(&self, content: &str) -> Result<Config, ConfigError> {
        toml::from_str(content).map_err(|e| ConfigError::TomlDe(e))
    }
    
    fn load_json(&self, content: &str) -> Result<Config, ConfigError> {
        serde_json::from_str(content).map_err(|e| ConfigError::Json(e))
    }
    
    fn load_yaml(&self, content: &str) -> Result<Config, ConfigError> {
        serde_yaml::from_str(content).map_err(|e| ConfigError::Yaml(e))
    }
}
