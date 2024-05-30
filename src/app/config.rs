use std::env;
use std::net::SocketAddr;
use std::str::FromStr;

use dotenvy::dotenv;

#[derive(Debug)]
pub struct Config {
    /// Listen Address
    listen_address: SocketAddr,

    /// Logging Level
    log_level: tracing::Level,
    // TODO: implement other config options here
}

impl Config {
    pub fn from_env() -> Result<Config, ConfigError> {
        if dotenv().is_err() {
            tracing::warn!("No .env file found");
        }

        let listen_address_str = match env::var("LISTEN_ADDRESS") {
            Ok(address) => address,
            Err(_e) => {
                tracing::warn!("No LISTEN_ADDRESS found in .env. Using default");
                "::1:3000".to_string()
            }
        };
        let listen_address = listen_address_str.parse()?;

        let log_level_str = match env::var("LOG_LEVEL") {
            Ok(level) => level,
            Err(_e) => {
                tracing::warn!("No LOG_LEVEL found in .env. Using default");
                "info".to_string()
            }
        };
        let log_level = match tracing::Level::from_str(&log_level_str) {
            Ok(level) => level,
            Err(_e) => {
                tracing::warn!("Invalid LOG_LEVEL found in .env. Using default");
                tracing::Level::INFO
            }
        };

        // TODO: Implement other config building here

        Ok(Config {
            listen_address,
            log_level,
        })
    }

    pub fn listen_address(&self) -> &SocketAddr {
        &self.listen_address
    }

    pub fn log_level(&self) -> &tracing::Level {
        &self.log_level
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Missing Env: {0}")]
    InvalidEnv(#[from] env::VarError),
    #[error("Invalid Listen Address: {0}")]
    InvalidListenAddress(#[from] std::net::AddrParseError),
    // TODO: and add config related errors here
}
