# SigmaOS Config System

SigmaOS declarative configuration system providing SigmaOS config system design.

## Features

- **Declarative Configuration** - Define system configuration declaratively
- **Multiple Formats** - Support for TOML, JSON, and YAML
- **Validation** - Comprehensive configuration validation
- **Defaults** - Sensible default values
- **Hot Reload** - Configuration reload without restart
- **Schema Validation** - Type-safe configuration schema
- **Environment Variables** - Support for environment variable substitution
- **Hierarchical Config** - Support for configuration inheritance

## Installation

```bash
cargo install sigmaos-config-system
```

## Usage

### Generate Default Configuration

```bash
sigmaos-config generate --output ~/.config/sigmaos
```

### Load Configuration

```bash
sigmaos-config load --file ~/.config/sigmaos/config.toml
```

### Validate Configuration

```bash
sigmaos-config validate --file ~/.config/sigmaos/config.toml
```

### Show Configuration

```bash
sigmaos-config show
```

### Set Configuration Value

```bash
sigmaos-config set desktop.theme sigmaos-light
sigmaos-config set window_manager.type hyprland
sigmaos-config set terminal.emulator kitty
```

### Get Configuration Value

```bash
sigmaos-config get desktop.theme
sigmaos-config get window_manager.type
sigmaos-config get terminal.emulator
```

## Configuration Format

### TOML Example

```toml
[desktop]
theme = "sigmaos-dark"
icon_theme = "Papirus-Dark"
cursor_theme = "Bibata-Modern-Amber"
wallpaper = "/usr/share/sigmaos/wallpapers/default.jpg"
font_family = "Inter"
font_size = 12
transparency = 0.8
animations = true
animation_speed = "normal"

[window_manager]
type = "hyprland"
layout = "dwindle"
gaps = 5
border_size = 2
focus_follows_mouse = true
mouse_warping = false

[terminal]
emulator = "kitty"
font_family = "JetBrains Mono"
font_size = 12
color_scheme = "sigmaos-dark"
opacity = 0.95
padding = 10
cursor_style = "block"
cursor_blink = true

[shell]
type = "zsh"
prompt_theme = "sigmaos"
history_size = 10000
auto_complete = true
syntax_highlighting = true
plugins = ["zsh-autosuggestions", "zsh-syntax-highlighting"]

[network]
hostname = "sigmaos"
domain = "local"
dns_servers = ["8.8.8.8", "8.8.4.4"]

[power]
power_profile = "balanced"
screen_timeout = 300
sleep_timeout = 600
hibernate_timeout = 1800
battery_saver = false
auto_brightness = true

[accessibility]
screen_reader = false
high_contrast = false
large_text = false
text_scale = 1.0
reduce_motion = false
focus_indicators = true
keyboard_navigation = true

[performance]
gpu_acceleration = true
compositing = true
memory_management = "balanced"
cpu_optimization = "performance"
io_optimization = "balanced"
startup_time = "fast"
resource_usage = "low"
battery_life = "optimized"

[security]
selinux = true
apparmor = false
firewall = true
encryption = true
audit_logging = true
secure_boot = true
disk_encryption = false

[updates]
enabled = true
auto_update = false
update_channel = "stable"
check_interval = 86400
notify_updates = true

[logging]
level = "info"
file = "/var/log/sigmaos.log"
max_size = 10485760
max_files = 5
syslog = true
journal = true
```

## Configuration Sections

### Desktop
- Theme, icon theme, cursor theme
- Wallpaper and fonts
- Transparency and animations

### Window Manager
- Window manager type and layout
- Gaps and borders
- Focus and mouse settings
- Keybindings

### Terminal
- Terminal emulator and fonts
- Color scheme and opacity
- Cursor settings

### Shell
- Shell type and prompt
- History and completion
- Plugins

### Network
- Hostname and domain
- DNS servers
- Proxy and firewall

### Power
- Power profiles
- Timeouts
- Battery settings

### Accessibility
- Screen reader
- High contrast
- Large text
- Keyboard navigation

### Performance
- GPU acceleration
- Compositing
- Memory management
- CPU optimization

### Security
- SELinux/AppArmor
- Firewall
- Encryption
- Audit logging

### Updates
- Update settings
- Channels
- Intervals

### Logging
- Log levels
- File settings
- Syslog and journal

## Environment Variables

Configuration supports environment variable substitution:

```toml
[desktop]
wallpaper = "${HOME}/Pictures/wallpaper.jpg"
font_family = "${SIGMAOS_FONT:-Inter}"
```

## Validation

Configuration is validated against a schema:

- Type checking
- Range validation
- Enum validation
- Required fields
- Dependencies

## Error Handling

All errors are handled gracefully with detailed error messages:

- File not found
- Parse errors
- Validation errors
- Permission errors

## License

GPL-3.0-or-later
