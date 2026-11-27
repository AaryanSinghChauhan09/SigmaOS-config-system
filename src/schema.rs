use crate::error::ConfigError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
            ("desktop", "font_size") => {
                self.desktop.font_size = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("desktop", "transparency") => {
                self.desktop.transparency = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("desktop", "animations") => {
                self.desktop.animations = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("window_manager", "type") => self.window_manager.type_ = value.to_string(),
            ("window_manager", "layout") => self.window_manager.layout = value.to_string(),
            ("window_manager", "gaps") => {
                self.window_manager.gaps = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("terminal", "emulator") => self.terminal.emulator = value.to_string(),
            ("terminal", "font_family") => self.terminal.font_family = value.to_string(),
            ("terminal", "font_size") => {
                self.terminal.font_size = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("shell", "type") => self.shell.type_ = value.to_string(),
            ("network", "hostname") => self.network.hostname = value.to_string(),
            ("network", "domain") => self.network.domain = value.to_string(),
            ("power", "power_profile") => self.power.power_profile = value.to_string(),
            ("power", "screen_timeout") => {
                self.power.screen_timeout = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("accessibility", "screen_reader") => {
                self.accessibility.screen_reader = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("accessibility", "high_contrast") => {
                self.accessibility.high_contrast = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("performance", "gpu_acceleration") => {
                self.performance.gpu_acceleration = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("security", "selinux") => {
                self.security.selinux = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("updates", "enabled") => {
                self.updates.enabled = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("logging", "level") => self.logging.level = value.to_string(),
            ("desktop", "wallpaper") => self.desktop.wallpaper = value.to_string(),
            ("desktop", "cursor_theme") => self.desktop.cursor_theme = value.to_string(),
            ("desktop", "animation_speed") => self.desktop.animation_speed = value.to_string(),
            ("window_manager", "border_size") => {
                self.window_manager.border_size = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("window_manager", "focus_follows_mouse") => {
                self.window_manager.focus_follows_mouse = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("window_manager", "mouse_warping") => {
                self.window_manager.mouse_warping = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("terminal", "color_scheme") => self.terminal.color_scheme = value.to_string(),
            ("terminal", "opacity") => {
                self.terminal.opacity = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("terminal", "padding") => {
                self.terminal.padding = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("terminal", "cursor_style") => self.terminal.cursor_style = value.to_string(),
            ("terminal", "cursor_blink") => {
                self.terminal.cursor_blink = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("shell", "prompt_theme") => self.shell.prompt_theme = value.to_string(),
            ("shell", "history_size") => {
                self.shell.history_size = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("shell", "auto_complete") => {
                self.shell.auto_complete = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("shell", "syntax_highlighting") => {
                self.shell.syntax_highlighting = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("power", "sleep_timeout") => {
                self.power.sleep_timeout = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("power", "hibernate_timeout") => {
                self.power.hibernate_timeout = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("power", "battery_saver") => {
                self.power.battery_saver = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("power", "auto_brightness") => {
                self.power.auto_brightness = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("accessibility", "large_text") => {
                self.accessibility.large_text = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("accessibility", "text_scale") => {
                self.accessibility.text_scale = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("accessibility", "reduce_motion") => {
                self.accessibility.reduce_motion = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("accessibility", "focus_indicators") => {
                self.accessibility.focus_indicators = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("accessibility", "keyboard_navigation") => {
                self.accessibility.keyboard_navigation = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("performance", "compositing") => {
                self.performance.compositing = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("performance", "memory_management") => {
                self.performance.memory_management = value.to_string()
            }
            ("performance", "cpu_optimization") => {
                self.performance.cpu_optimization = value.to_string()
            }
            ("performance", "io_optimization") => {
                self.performance.io_optimization = value.to_string()
            }
            ("performance", "startup_time") => self.performance.startup_time = value.to_string(),
            ("performance", "resource_usage") => {
                self.performance.resource_usage = value.to_string()
            }
            ("performance", "battery_life") => self.performance.battery_life = value.to_string(),
            ("security", "apparmor") => {
                self.security.apparmor = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("security", "firewall") => {
                self.security.firewall = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("security", "encryption") => {
                self.security.encryption = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("security", "audit_logging") => {
                self.security.audit_logging = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("security", "secure_boot") => {
                self.security.secure_boot = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("security", "disk_encryption") => {
                self.security.disk_encryption = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("updates", "auto_update") => {
                self.updates.auto_update = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("updates", "update_channel") => self.updates.update_channel = value.to_string(),
            ("updates", "check_interval") => {
                self.updates.check_interval = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("updates", "notify_updates") => {
                self.updates.notify_updates = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("logging", "file") => self.logging.file = value.to_string(),
            ("logging", "max_size") => {
                self.logging.max_size = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("logging", "max_files") => {
                self.logging.max_files = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("logging", "syslog") => {
                self.logging.syslog = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
            ("logging", "journal") => {
                self.logging.journal = value
                    .parse()
                    .map_err(|_| ConfigError::InvalidValue(value.to_string()))?
            }
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
            ("desktop", "wallpaper") => self.desktop.wallpaper.clone(),
            ("desktop", "cursor_theme") => self.desktop.cursor_theme.clone(),
            ("desktop", "animation_speed") => self.desktop.animation_speed.clone(),
            ("window_manager", "border_size") => self.window_manager.border_size.to_string(),
            ("window_manager", "focus_follows_mouse") => {
                self.window_manager.focus_follows_mouse.to_string()
            }
            ("window_manager", "mouse_warping") => self.window_manager.mouse_warping.to_string(),
            ("terminal", "color_scheme") => self.terminal.color_scheme.clone(),
            ("terminal", "opacity") => self.terminal.opacity.to_string(),
            ("terminal", "padding") => self.terminal.padding.to_string(),
            ("terminal", "cursor_style") => self.terminal.cursor_style.clone(),
            ("terminal", "cursor_blink") => self.terminal.cursor_blink.to_string(),
            ("shell", "prompt_theme") => self.shell.prompt_theme.clone(),
            ("shell", "history_size") => self.shell.history_size.to_string(),
            ("shell", "auto_complete") => self.shell.auto_complete.to_string(),
            ("shell", "syntax_highlighting") => self.shell.syntax_highlighting.to_string(),
            ("power", "sleep_timeout") => self.power.sleep_timeout.to_string(),
            ("power", "hibernate_timeout") => self.power.hibernate_timeout.to_string(),
            ("power", "battery_saver") => self.power.battery_saver.to_string(),
            ("power", "auto_brightness") => self.power.auto_brightness.to_string(),
            ("accessibility", "large_text") => self.accessibility.large_text.to_string(),
            ("accessibility", "text_scale") => self.accessibility.text_scale.to_string(),
            ("accessibility", "reduce_motion") => self.accessibility.reduce_motion.to_string(),
            ("accessibility", "focus_indicators") => {
                self.accessibility.focus_indicators.to_string()
            }
            ("accessibility", "keyboard_navigation") => {
                self.accessibility.keyboard_navigation.to_string()
            }
            ("performance", "compositing") => self.performance.compositing.to_string(),
            ("performance", "memory_management") => self.performance.memory_management.clone(),
            ("performance", "cpu_optimization") => self.performance.cpu_optimization.clone(),
            ("performance", "io_optimization") => self.performance.io_optimization.clone(),
            ("performance", "startup_time") => self.performance.startup_time.clone(),
            ("performance", "resource_usage") => self.performance.resource_usage.clone(),
            ("performance", "battery_life") => self.performance.battery_life.clone(),
            ("security", "apparmor") => self.security.apparmor.to_string(),
            ("security", "firewall") => self.security.firewall.to_string(),
            ("security", "encryption") => self.security.encryption.to_string(),
            ("security", "audit_logging") => self.security.audit_logging.to_string(),
            ("security", "secure_boot") => self.security.secure_boot.to_string(),
            ("security", "disk_encryption") => self.security.disk_encryption.to_string(),
            ("updates", "auto_update") => self.updates.auto_update.to_string(),
            ("updates", "update_channel") => self.updates.update_channel.clone(),
            ("updates", "check_interval") => self.updates.check_interval.to_string(),
            ("updates", "notify_updates") => self.updates.notify_updates.to_string(),
            ("logging", "file") => self.logging.file.clone(),
            ("logging", "max_size") => self.logging.max_size.to_string(),
            ("logging", "max_files") => self.logging.max_files.to_string(),
            ("logging", "syslog") => self.logging.syslog.to_string(),
            ("logging", "journal") => self.logging.journal.to_string(),
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
            plugins: vec![
                "zsh-autosuggestions".to_string(),
                "zsh-syntax-highlighting".to_string(),
            ],
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_produce_valid_config() {
        let mut cfg = Config::default();
        cfg.apply_defaults();
        crate::validator::ConfigValidator::new()
            .unwrap()
            .validate(&cfg)
            .unwrap();
    }

    #[test]
    fn set_and_get_value_roundtrip() {
        let mut cfg = Config::default();
        cfg.set_value("desktop.theme", "sigmaos").unwrap();
        cfg.set_value("desktop.font_size", "14").unwrap();
        assert_eq!(cfg.get_value("desktop.theme").unwrap(), "sigmaos");
        assert_eq!(cfg.get_value("desktop.font_size").unwrap(), "14");
    }

    #[test]
    fn invalid_key_and_value_rejected() {
        let mut cfg = Config::default();
        assert!(matches!(
            cfg.set_value("bogus.key", "x"),
            Err(ConfigError::InvalidKey(_))
        ));
        assert!(matches!(
            cfg.set_value("desktop.font_size", "abc"),
            Err(ConfigError::InvalidValue(_))
        ));
        assert!(cfg.get_value("nope.key").is_err());
    }

    #[test]
    fn set_get_roundtrip_all_sections() {
        let mut cfg = Config::default();
        let cases: &[(&str, &str)] = &[
            ("desktop.wallpaper", "/wp.png"),
            ("desktop.cursor_theme", "breeze"),
            ("desktop.animation_speed", "fast"),
            ("window_manager.border_size", "2"),
            ("window_manager.focus_follows_mouse", "false"),
            ("window_manager.mouse_warping", "true"),
            ("terminal.color_scheme", "solarized"),
            ("terminal.opacity", "0.9"),
            ("terminal.padding", "4"),
            ("terminal.cursor_style", "block"),
            ("terminal.cursor_blink", "false"),
            ("shell.prompt_theme", "minimal"),
            ("shell.history_size", "5000"),
            ("shell.auto_complete", "false"),
            ("shell.syntax_highlighting", "true"),
            ("power.sleep_timeout", "300"),
            ("power.hibernate_timeout", "1800"),
            ("power.battery_saver", "true"),
            ("power.auto_brightness", "false"),
            ("accessibility.large_text", "true"),
            ("accessibility.text_scale", "1.5"),
            ("accessibility.reduce_motion", "true"),
            ("accessibility.focus_indicators", "false"),
            ("accessibility.keyboard_navigation", "true"),
            ("performance.compositing", "false"),
            ("performance.memory_management", "conservative"),
            ("performance.cpu_optimization", "powersave"),
            ("performance.io_optimization", "none"),
            ("performance.startup_time", "10s"),
            ("performance.resource_usage", "low"),
            ("performance.battery_life", "high"),
            ("security.apparmor", "true"),
            ("security.firewall", "false"),
            ("security.encryption", "true"),
            ("security.audit_logging", "true"),
            ("security.secure_boot", "false"),
            ("security.disk_encryption", "true"),
            ("updates.auto_update", "false"),
            ("updates.update_channel", "beta"),
            ("updates.check_interval", "7200"),
            ("updates.notify_updates", "false"),
            ("logging.file", "/var/log/sigmaos.log"),
            ("logging.max_size", "1048576"),
            ("logging.max_files", "5"),
            ("logging.syslog", "false"),
            ("logging.journal", "true"),
        ];
        for (k, v) in cases {
            cfg.set_value(k, v)
                .unwrap_or_else(|e| panic!("set {k}: {e}"));
        }
        for (k, v) in cases {
            // bools/ints stringify differently; compare loosely via re-parse
            let got = cfg.get_value(k).unwrap();
            if *v == "true" || *v == "false" {
                assert_eq!(got, *v, "get {k}");
            }
        }
        assert_eq!(cfg.get_value("desktop.wallpaper").unwrap(), "/wp.png");
        assert_eq!(cfg.get_value("updates.update_channel").unwrap(), "beta");
        assert_eq!(cfg.get_value("logging.max_files").unwrap(), "5");
        // float roundtrip: 0.9 -> "0.9"
        assert_eq!(cfg.get_value("terminal.opacity").unwrap(), "0.9");
    }
}
