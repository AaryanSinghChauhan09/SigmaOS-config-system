#![forbid(unsafe_code)]

pub mod config;
pub mod error;
pub mod loader;
pub mod schema;
pub mod validator;

pub use config::ConfigManager;
pub use error::ConfigError;
pub use schema::Config;
