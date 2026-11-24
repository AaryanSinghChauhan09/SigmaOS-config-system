use crate::error::ConfigError;
use crate::schema::Config;
use crate::loader::ConfigLoader;
use crate::validator::ConfigValidator;
use std::path::PathBuf;
use std::fs;

pub struct ConfigManager {
    loader: ConfigLoader,
    validator: ConfigValidator,
}

impl ConfigManager {
    pub fn new() -> Result<Self, ConfigError> {
        Ok(Self {
            loader: ConfigLoader::new()?,
            validator: ConfigValidator::new()?,
        })
    }
    
    pub fn load_config(&self, file: Option<PathBuf>, config_dir: PathBuf) -> Result<Config, ConfigError> {
        let config_path = file.unwrap_or_else(|| config_dir.join("config.toml"));
        
        // Load configuration
        let mut config = self.loader.load(&config_path)?;
        
        // Apply defaults
        config.apply_defaults();
        
        // Validate configuration
        self.validator.validate(&config)?;
        
        Ok(config)
    }
    
    pub fn validate_config(&self, file: &PathBuf) -> Result<(), ConfigError> {
        let config = self.loader.load(file)?;
        self.validator.validate(&config)?;
        Ok(())
    }
    
    pub fn generate_default_config(&self, output_dir: &PathBuf) -> Result<(), ConfigError> {
        fs::create_dir_all(output_dir)?;
        
        let default_config = Config::default();
        let config_path = output_dir.join("config.toml");
        
        let toml_content = toml::to_string_pretty(&default_config)?;
        fs::write(&config_path, toml_content)?;
        
        log::info!("Generated default configuration at {}", config_path.display());
        Ok(())
    }
    
    pub fn set_value(&self, key: &str, value: &str, file: Option<PathBuf>) -> Result<(), ConfigError> {
        let config_path = file.unwrap_or_else(|| {
            directories::ProjectDirs::from("org", "sigmaos", "sigmaos").map(|p| p.config_dir().to_path_buf()).unwrap_or_else(|| PathBuf::from("~/.config"))
                .join("sigmaos")
                .join("config.toml")
        });
        
        let mut config = self.loader.load(&config_path)?;
        config.set_value(key, value)?;
        
        let toml_content = toml::to_string_pretty(&config)?;
        fs::write(&config_path, toml_content)?;
        
        log::info!("Set configuration value: {} = {}", key, value);
        Ok(())
    }
    
    pub fn get_value(&self, key: &str, file: Option<PathBuf>) -> Result<String, ConfigError> {
        let config_path = file.unwrap_or_else(|| {
            directories::ProjectDirs::from("org", "sigmaos", "sigmaos").map(|p| p.config_dir().to_path_buf()).unwrap_or_else(|| PathBuf::from("~/.config"))
                .join("sigmaos")
                .join("config.toml")
        });
        
        let config = self.loader.load(&config_path)?;
        config.get_value(key)
    }
}
