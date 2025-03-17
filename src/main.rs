mod config;
mod database;

use anyhow::Result;
use config::Config;
use database::Database;

#[tokio::main]
async fn main() -> Result<()> {
    // Load the application configuration
    let config = Config::new();

    // Initialize the database connection
    let db = Database::new(&config).await?;

    // Test the connection
    db.test_connection()
        .await?;

    Ok(())
}
