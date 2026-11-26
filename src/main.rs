use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod config;
mod error;
mod loader;
mod schema;
mod validator;

use config::ConfigManager;
use error::ConfigError;

#[derive(Parser)]
#[command(name = "sigmaos-config")]
#[command(about = "SigmaOS declarative configuration system")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Load and validate configuration
    Load {
        /// Configuration file path
        #[arg(short, long)]
        file: Option<PathBuf>,

        /// Configuration directory
        #[arg(short, long, default_value = "~/.config/sigmaos")]
        config_dir: PathBuf,
    },
    /// Validate configuration
    Validate {
        /// Configuration file path
        #[arg(short, long)]
        file: PathBuf,
    },
    /// Generate default configuration
    Generate {
        /// Output directory
        #[arg(short, long, default_value = "~/.config/sigmaos")]
        output: PathBuf,
    },
    /// Show current configuration
    Show {
        /// Configuration file path
        #[arg(short, long)]
        file: Option<PathBuf>,
    },
    /// Set configuration value
    Set {
        /// Configuration key (e.g., "desktop.theme")
        key: String,
        /// Configuration value
        value: String,
        /// Configuration file path
        #[arg(short, long)]
        file: Option<PathBuf>,
    },
    /// Get configuration value
    Get {
        /// Configuration key (e.g., "desktop.theme")
        key: String,
        /// Configuration file path
        #[arg(short, long)]
        file: Option<PathBuf>,
    },
}

fn main() -> Result<(), ConfigError> {
    env_logger::init();

    let cli = Cli::parse();
    let config_manager = ConfigManager::new()?;

    match cli.command {
        Commands::Load { file, config_dir } => {
            let config = config_manager.load_config(file, config_dir)?;
            println!("Configuration loaded successfully");
            println!("{}", serde_json::to_string_pretty(&config)?);
        }
        Commands::Validate { file } => {
            config_manager.validate_config(&file)?;
            println!("Configuration is valid");
        }
        Commands::Generate { output } => {
            config_manager.generate_default_config(&output)?;
            println!("Default configuration generated in {}", output.display());
        }
        Commands::Show { file } => {
            let config = config_manager.load_config(file, PathBuf::from("~/.config/sigmaos"))?;
            println!("{}", serde_json::to_string_pretty(&config)?);
        }
        Commands::Set { key, value, file } => {
            config_manager.set_value(&key, &value, file)?;
            println!("Configuration value set: {} = {}", key, value);
        }
        Commands::Get { key, file } => {
            let value = config_manager.get_value(&key, file)?;
            println!("{}", value);
        }
    }

    Ok(())
}
