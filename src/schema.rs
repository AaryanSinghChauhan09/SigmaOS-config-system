use crate::error::ConfigError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub desktop: DesktopConfig,
    pub window_manager: WindowManagerConfig,
    pub terminal: TerminalConfig,
    pub shell: ShellConfig,
    pub network: NetworkConfig,
    pub power: PowerConfig,
    pub accessibility: AccessibilityConfig,
    pub performance: PerformanceConfig,
    pub security: SecurityConfig,
    pub updates: UpdateConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesktopConfig {
    pub theme: String,
    pub icon_theme: String,
    pub cursor_theme: String,
    pub wallpaper: String,
    pub font_family: String,
    pub font_size: u32,
    pub transparency: f32,
    pub animations: bool,
    pub animation_speed: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowManagerConfig {
    pub type_: String,
    pub layout: String,
    pub gaps: u32,
    pub border_size: u32,
    pub focus_follows_mouse: bool,
    pub mouse_warping: bool,
    pub keybindings: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalConfig {
    pub emulator: String,
    pub font_family: String,
    pub font_size: u32,
    pub color_scheme: String,
    pub opacity: f32,
    pub padding: u32,
    pub cursor_style: String,
    pub cursor_blink: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShellConfig {
    pub type_: String,
    pub prompt_theme: String,
    pub history_size: u32,
    pub auto_complete: bool,
    pub syntax_highlighting: bool,
    pub plugins: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub hostname: String,
    pub domain: String,
    pub dns_servers: Vec<String>,
    pub proxy: Option<ProxyConfig>,
    pub firewall: FirewallConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    pub type_: String,
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallConfig {
    pub enabled: bool,
    pub default_policy: String,
    pub allowed_ports: Vec<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerConfig {
    pub power_profile: String,
    pub screen_timeout: u32,
    pub sleep_timeout: u32,
    pub hibernate_timeout: u32,
    pub battery_saver: bool,
    pub auto_brightness: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityConfig {
    pub screen_reader: bool,
    pub high_contrast: bool,
    pub large_text: bool,
    pub text_scale: f32,
    pub reduce_motion: bool,
    pub focus_indicators: bool,
    pub keyboard_navigation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub gpu_acceleration: bool,
    pub compositing: bool,
    pub memory_management: String,
    pub cpu_optimization: String,
    pub io_optimization: String,
    pub startup_time: String,
    pub resource_usage: String,
    pub battery_life: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub selinux: bool,
    pub apparmor: bool,
    pub firewall: bool,
    pub encryption: bool,
    pub audit_logging: bool,
    pub secure_boot: bool,
    pub disk_encryption: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConfig {
    pub enabled: bool,
    pub auto_update: bool,
    pub update_channel: String,
    pub check_interval: u32,
    pub notify_updates: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub file: String,
    pub max_size: u64,
    pub max_files: u32,
    pub syslog: bool,
    pub journal: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            desktop: DesktopConfig::default(),
            window_manager: WindowManagerConfig::default(),
            terminal: TerminalConfig::default(),
            shell: ShellConfig::default(),
            network: NetworkConfig::default(),
            power: PowerConfig::default(),
            accessibility: AccessibilityConfig::default(),
            performance: PerformanceConfig::default(),
            security: SecurityConfig::default(),
            updates: UpdateConfig::default(),
            logging: LoggingConfig::default(),
        }
    }
}

impl Config {
    pub fn apply_defaults(&mut self) {
        // Apply default values for missing fields
        if self.desktop.theme.is_empty() {
            self.desktop.theme = "sigmaos-dark".to_string();
        }
        if self.window_manager.type_.is_empty() {
            self.window_manager.type_ = "hyprland".to_string();
        }
        if self.terminal.emulator.is_empty() {
            self.terminal.emulator = "kitty".to_string();
        }
        if self.shell.type_.is_empty() {
            self.shell.type_ = "zsh".to_string();
        }
    }
    
    pub fn set_value(&mut self, key: &str, value: &str) -> Result<(), ConfigError> {
        let parts: Vec<&str> = key.split('.').collect();
        if parts.len() < 2 {
            return Err(ConfigError::InvalidKey(key.to_string()));
        }
        
        match (parts[0], parts[1]) {
            ("desktop", "theme") => self.desktop.theme = value.to_string(),
            ("desktop", "icon_theme") => self.desktop.icon_theme = value.to_string(),
            ("desktop", "font_family") => self.desktop.font_family = value.to_string(),
            ("desktop", "font_size") => self.desktop.font_size = value.parse().map_err(|_| ConfigError::InvalidValue(value.to_string()))?,
            ("desktop", "transparency") => self.desktop.transparency = value.parse().map_err(|_| ConfigError::InvalidValue(value.to_string()))?,
            ("desktop", "animations") => self.desktop.animations = value.parse().map_err(|_| ConfigError::InvalidValue(value.to_string()))?,
            ("window_manager", "type") => self.window_manager.type_ = value.to_string(),
            ("window_manager", "layout") => self.window_manager.layout = value.to_string(),
            ("window_manager", "gaps") => self.window_manager.gaps = value.parse().map_err(|_| ConfigError::InvalidValue(value.to_string()))?,
            ("terminal", "emulator") => self.terminal.emulator = value.to_string(),
            ("terminal", "font_family") => self.terminal.font_family = value.to_string(),
            ("terminal", "font_size") => self.terminal.font_size = value.parse().map_err(|_| ConfigError::InvalidValue(value.to_string()))?,
            ("shell", "type") => self.shell.type_ = value.to_string(),
            ("network", "hostname") => self.network.hostname = value.to_string(),
            ("network", "domain") => self.network.domain = value.to_string(),
            ("power", "power_profile") => self.power.power_profile = value.to_string(),
            ("power", "screen_timeout") => self.power.screen_timeout = value.parse().map_err(|_| ConfigError::InvalidValue(value.to_string()))?,
            ("accessibility", "screen_reader") => self.accessibility.screen_reader = value.parse().map_err(|_| ConfigError::InvalidValue(value.to_string()))?,
            ("accessibility", "high_contrast") => self.accessibility.high_contrast = value.parse().map_err(|_| ConfigError::InvalidValue(value.to_string()))?,
            ("performance", "gpu_acceleration") => self.performance.gpu_acceleration = value.parse().map_err(|_| ConfigError::InvalidValue(value.to_string()))?,
            ("security", "selinux") => self.security.selinux = value.parse().map_err(|_| ConfigError::InvalidValue(value.to_string()))?,
            ("updates", "enabled") => self.updates.enabled = value.parse().map_err(|_| ConfigError::InvalidValue(value.to_string()))?,
            ("logging", "level") => self.logging.level = value.to_string(),
            _ => return Err(ConfigError::InvalidKey(key.to_string())),
        }
        
        Ok(())
    }
    
    pub fn get_value(&self, key: &str) -> Result<String, ConfigError> {
        let parts: Vec<&str> = key.split('.').collect();
        if parts.len() < 2 {
            return Err(ConfigError::InvalidKey(key.to_string()));
        }
        
        let value = match (parts[0], parts[1]) {
            ("desktop", "theme") => self.desktop.theme.clone(),
            ("desktop", "icon_theme") => self.desktop.icon_theme.clone(),
            ("desktop", "font_family") => self.desktop.font_family.clone(),
            ("desktop", "font_size") => self.desktop.font_size.to_string(),
            ("desktop", "transparency") => self.desktop.transparency.to_string(),
            ("desktop", "animations") => self.desktop.animations.to_string(),
            ("window_manager", "type") => self.window_manager.type_.clone(),
            ("window_manager", "layout") => self.window_manager.layout.clone(),
            ("window_manager", "gaps") => self.window_manager.gaps.to_string(),
            ("terminal", "emulator") => self.terminal.emulator.clone(),
            ("terminal", "font_family") => self.terminal.font_family.clone(),
            ("terminal", "font_size") => self.terminal.font_size.to_string(),
            ("shell", "type") => self.shell.type_.clone(),
            ("network", "hostname") => self.network.hostname.clone(),
            ("network", "domain") => self.network.domain.clone(),
            ("power", "power_profile") => self.power.power_profile.clone(),
            ("power", "screen_timeout") => self.power.screen_timeout.to_string(),
            ("accessibility", "screen_reader") => self.accessibility.screen_reader.to_string(),
            ("accessibility", "high_contrast") => self.accessibility.high_contrast.to_string(),
            ("performance", "gpu_acceleration") => self.performance.gpu_acceleration.to_string(),
            ("security", "selinux") => self.security.selinux.to_string(),
            ("updates", "enabled") => self.updates.enabled.to_string(),
            ("logging", "level") => self.logging.level.clone(),
            _ => return Err(ConfigError::InvalidKey(key.to_string())),
        };
        
        Ok(value)
    }
}

impl Default for DesktopConfig {
    fn default() -> Self {
        Self {
            theme: "sigmaos-dark".to_string(),
            icon_theme: "Papirus-Dark".to_string(),
            cursor_theme: "Bibata-Modern-Amber".to_string(),
            wallpaper: "/usr/share/sigmaos/wallpapers/default.jpg".to_string(),
            font_family: "Inter".to_string(),
            font_size: 12,
            transparency: 0.8,
            animations: true,
            animation_speed: "normal".to_string(),
        }
    }
}

impl Default for WindowManagerConfig {
    fn default() -> Self {
        Self {
            type_: "hyprland".to_string(),
            layout: "dwindle".to_string(),
            gaps: 5,
            border_size: 2,
            focus_follows_mouse: true,
            mouse_warping: false,
            keybindings: HashMap::new(),
        }
    }
}

impl Default for TerminalConfig {
    fn default() -> Self {
        Self {
            emulator: "kitty".to_string(),
            font_family: "JetBrains Mono".to_string(),
            font_size: 12,
            color_scheme: "sigmaos-dark".to_string(),
            opacity: 0.95,
            padding: 10,
            cursor_style: "block".to_string(),
            cursor_blink: true,
        }
    }
}

impl Default for ShellConfig {
    fn default() -> Self {
        Self {
            type_: "zsh".to_string(),
            prompt_theme: "sigmaos".to_string(),
            history_size: 10000,
            auto_complete: true,
            syntax_highlighting: true,
            plugins: vec!["zsh-autosuggestions".to_string(), "zsh-syntax-highlighting".to_string()],
        }
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            hostname: "sigmaos".to_string(),
            domain: "local".to_string(),
            dns_servers: vec!["8.8.8.8".to_string(), "8.8.4.4".to_string()],
            proxy: None,
            firewall: FirewallConfig::default(),
        }
    }
}

impl Default for FirewallConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            default_policy: "deny".to_string(),
            allowed_ports: vec![22, 80, 443],
        }
    }
}

impl Default for PowerConfig {
    fn default() -> Self {
        Self {
            power_profile: "balanced".to_string(),
            screen_timeout: 300,
            sleep_timeout: 600,
            hibernate_timeout: 1800,
            battery_saver: false,
            auto_brightness: true,
        }
    }
}

impl Default for AccessibilityConfig {
    fn default() -> Self {
        Self {
            screen_reader: false,
            high_contrast: false,
            large_text: false,
            text_scale: 1.0,
            reduce_motion: false,
            focus_indicators: true,
            keyboard_navigation: true,
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            gpu_acceleration: true,
            compositing: true,
            memory_management: "balanced".to_string(),
            cpu_optimization: "performance".to_string(),
            io_optimization: "balanced".to_string(),
            startup_time: "fast".to_string(),
            resource_usage: "low".to_string(),
            battery_life: "optimized".to_string(),
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            selinux: true,
            apparmor: false,
            firewall: true,
            encryption: true,
            audit_logging: true,
            secure_boot: true,
            disk_encryption: false,
        }
    }
}

impl Default for UpdateConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            auto_update: false,
            update_channel: "stable".to_string(),
            check_interval: 86400,
            notify_updates: true,
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            file: "/var/log/sigmaos.log".to_string(),
            max_size: 10485760,
            max_files: 5,
            syslog: true,
            journal: true,
        }
    }
}
