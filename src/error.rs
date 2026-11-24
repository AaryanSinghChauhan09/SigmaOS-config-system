use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    
    #[error("TOML error: {0}")]
    Toml(#[from] toml::ser::Error),
    
    #[error("TOML error: {0}")]
    TomlDe(#[from] toml::de::Error),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("Invalid key: {0}")]
    InvalidKey(String),
    
    #[error("Invalid value: {0}")]
    InvalidValue(String),
}
