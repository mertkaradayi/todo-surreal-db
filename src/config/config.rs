// src/config/config.rs

use crate::config::DatabaseConfig;

#[derive(Debug, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
}

impl Config {
    pub fn new() -> Self {
        Self {
            database: DatabaseConfig::from_env(),
        }
    }
}
