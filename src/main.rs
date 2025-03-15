mod config;
mod database;

use std::error::Error;
use config::Config;
use database::Database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load the application configuration
    let config = Config::new();

    // Initialize the database connection
    let db = Database::new(&config).await?;

    // Test the connection
    db.test_connection().await?;

    Ok(())
}
