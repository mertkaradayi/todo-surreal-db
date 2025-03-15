use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;

use super::error::{DatabaseError, DatabaseResult};
use crate::config::Config;

pub struct Database {
    client: Surreal<Client>,
}

impl Database {
    pub async fn new(config: &Config) -> DatabaseResult<Self> {
        let client = Surreal::new::<Ws>(
            &config
                .database
                .dburl,
        )
        .await
        .map_err(DatabaseError::ConnectionError)?;

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
            .map_err(DatabaseError::ConnectionError)?;

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
            .map_err(DatabaseError::ConnectionError)?;

        Ok(Self { client })
    }

    pub async fn test_connection(&self) -> DatabaseResult<()> {
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
            .map_err(DatabaseError::ConnectionError)?;

        dbg!(result);
        Ok(())
    }

    // Add a getter for the client if needed
    // pub fn client(&self) -> &Surreal<Client> {
    //     &self.client
    // }
}

