use anyhow::{Context, Result};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;

use crate::config::Config;

pub struct Database {
    client: Surreal<Client>,
}

impl Database {
    pub async fn new(config: &Config) -> Result<Self> {
        let client = Surreal::new::<Ws>(
            &config
                .database
                .dburl,
        )
        .await
        .context("Failed to create database connection")?;

        // Authenticate using credentials
        client
            .signin(Root {
                username: &config
                    .database
                    .username,
                password: &config
                    .database
                    .password,
            })
            .await
            .context("Failed to authenticate with database")?;

        // Select namespace and database
        client
            .use_ns(
                &config
                    .database
                    .namespace,
            )
            .use_db(
                &config
                    .database
                    .dbname,
            )
            .await
            .context("Failed to select namespace or database, or both")?;

        Ok(Self { client })
    }

    pub async fn test_connection(&self) -> Result<()> {
        let result = self
            .client
            .query(
                "
                RETURN 9; 
                RETURN 10; 
                SELECT * FROM { is: 'Nice database' };
            ",
            )
            .await
            .context("Failed to execute test query")?;

        dbg!(result);
        Ok(())
    }

    // Add a getter for the client if needed
    // pub fn client(&self) -> &Surreal<Client> {
    //     &self.client
    // }
}
