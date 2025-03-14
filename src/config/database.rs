// src/config/database.rs

use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub username: String,
    pub password: String,
    pub namespace: String,
    pub dbname: String,
}

impl DatabaseConfig {
    pub fn from_env() -> Self {
        dotenv().ok();

        Self {
            username: env::var("DB_USERNAME").expect("DB_USERNAME must be set"),
            password: env::var("DB_PASSWORD").expect("DB_PASSWORD must be set"),
            namespace: env::var("DB_NAMESPACE").expect("DB_NAMESPACE must be set"),
            dbname: env::var("DB_DBNAME").expect("DB_DBNAME must be set"),
        }
    }
}
