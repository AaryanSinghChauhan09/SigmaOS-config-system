use crate::error::ConfigError;
use crate::schema::Config;

pub struct ConfigValidator;

impl ConfigValidator {
    pub fn new() -> Result<Self, ConfigError> {
        Ok(Self)
    }
    
    pub fn validate(&self, config: &Config) -> Result<(), ConfigError> {
        self.validate_desktop(&config.desktop)?;
        self.validate_window_manager(&config.window_manager)?;
        self.validate_terminal(&config.terminal)?;
        self.validate_shell(&config.shell)?;
        self.validate_network(&config.network)?;
        self.validate_power(&config.power)?;
        self.validate_accessibility(&config.accessibility)?;
        self.validate_performance(&config.performance)?;
        self.validate_security(&config.security)?;
        self.validate_updates(&config.updates)?;
        self.validate_logging(&config.logging)?;
        
        Ok(())
    }
    
    fn validate_desktop(&self, config: &crate::schema::DesktopConfig) -> Result<(), ConfigError> {
        if config.font_size < 6 || config.font_size > 72 {
            return Err(ConfigError::Validation("Font size must be between 6 and 72".to_string()));
        }
        
        if config.transparency < 0.0 || config.transparency > 1.0 {
            return Err(ConfigError::Validation("Transparency must be between 0.0 and 1.0".to_string()));
        }
        
        Ok(())
    }
    
    fn validate_window_manager(&self, config: &crate::schema::WindowManagerConfig) -> Result<(), ConfigError> {
        let valid_types = ["hyprland", "sway", "i3", "dwm", "bspwm", "awesome", "qtile", "xmonad"];
        if !valid_types.contains(&config.type_.as_str()) {
            return Err(ConfigError::Validation(format!("Invalid window manager type: {}", config.type_)));
        }
        
        if config.gaps > 100 {
            return Err(ConfigError::Validation("Gaps must be less than 100".to_string()));
        }
        
        Ok(())
    }
    
    fn validate_terminal(&self, config: &crate::schema::TerminalConfig) -> Result<(), ConfigError> {
        if config.font_size < 6 || config.font_size > 72 {
            return Err(ConfigError::Validation("Terminal font size must be between 6 and 72".to_string()));
        }
        
        if config.opacity < 0.0 || config.opacity > 1.0 {
            return Err(ConfigError::Validation("Terminal opacity must be between 0.0 and 1.0".to_string()));
        }
        
        Ok(())
    }
    
    fn validate_shell(&self, config: &crate::schema::ShellConfig) -> Result<(), ConfigError> {
        let valid_types = ["zsh", "bash", "fish", "nushell", "xonsh"];
        if !valid_types.contains(&config.type_.as_str()) {
            return Err(ConfigError::Validation(format!("Invalid shell type: {}", config.type_)));
        }
        
        if config.history_size < 100 || config.history_size > 1000000 {
            return Err(ConfigError::Validation("History size must be between 100 and 1000000".to_string()));
        }
        
        Ok(())
    }
    
    fn validate_network(&self, config: &crate::schema::NetworkConfig) -> Result<(), ConfigError> {
        if config.hostname.is_empty() {
            return Err(ConfigError::Validation("Hostname cannot be empty".to_string()));
        }
        
        if config.dns_servers.is_empty() {
            return Err(ConfigError::Validation("At least one DNS server is required".to_string()));
        }
        
        Ok(())
    }
    
    fn validate_power(&self, config: &crate::schema::PowerConfig) -> Result<(), ConfigError> {
        let valid_profiles = ["power-saver", "balanced", "performance"];
        if !valid_profiles.contains(&config.power_profile.as_str()) {
            return Err(ConfigError::Validation(format!("Invalid power profile: {}", config.power_profile)));
        }
        
        if config.screen_timeout > 86400 {
            return Err(ConfigError::Validation("Screen timeout must be between 0 and 86400 seconds".to_string()));
        }
        
        Ok(())
    }
    
    fn validate_accessibility(&self, config: &crate::schema::AccessibilityConfig) -> Result<(), ConfigError> {
        if config.text_scale < 0.5 || config.text_scale > 3.0 {
            return Err(ConfigError::Validation("Text scale must be between 0.5 and 3.0".to_string()));
        }
        
        Ok(())
    }
    
    fn validate_performance(&self, config: &crate::schema::PerformanceConfig) -> Result<(), ConfigError> {
        let valid_memory = ["aggressive", "balanced", "conservative"];
        if !valid_memory.contains(&config.memory_management.as_str()) {
            return Err(ConfigError::Validation(format!("Invalid memory management: {}", config.memory_management)));
        }
        
        Ok(())
    }
    
    fn validate_security(&self, config: &crate::schema::SecurityConfig) -> Result<(), ConfigError> {
        // Security validation - ensure secure defaults
        if !config.selinux && !config.apparmor {
            return Err(ConfigError::Validation("At least one security module (SELinux or AppArmor) must be enabled".to_string()));
        }
        
        Ok(())
    }
    
    fn validate_updates(&self, config: &crate::schema::UpdateConfig) -> Result<(), ConfigError> {
        let valid_channels = ["stable", "beta", "nightly"];
        if !valid_channels.contains(&config.update_channel.as_str()) {
            return Err(ConfigError::Validation(format!("Invalid update channel: {}", config.update_channel)));
        }
        
        if config.check_interval < 3600 {
            return Err(ConfigError::Validation("Update check interval must be at least 3600 seconds".to_string()));
        }
        
        Ok(())
    }
    
    fn validate_logging(&self, config: &crate::schema::LoggingConfig) -> Result<(), ConfigError> {
        let valid_levels = ["error", "warn", "info", "debug", "trace"];
        if !valid_levels.contains(&config.level.as_str()) {
            return Err(ConfigError::Validation(format!("Invalid log level: {}", config.level)));
        }
        
        if config.max_size < 1024 {
            return Err(ConfigError::Validation("Max log size must be at least 1024 bytes".to_string()));
        }
        
        Ok(())
    }
}
