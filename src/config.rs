use std::env;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Error reading environment variable: {0}")]
    EnvVarError(String),
}

#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub server_port: u16, // Assuming you want a server port
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let database_url = env::var("DATABASE_URL")
            .map_err(|err| ConfigError::EnvVarError(format!("DATABASE_URL: {}", err)))?;

        let server_port_str = env::var("SERVER_PORT")
            .map_err(|err| ConfigError::EnvVarError(format!("SERVER_PORT: {}", err)))?;

        let server_port = server_port_str.parse::<u16>()
            .map_err(|err| ConfigError::EnvVarError(format!("Invalid server port: {}", err)))?;

        Ok(Config {
            database_url,
            server_port,
        })
    }
}
