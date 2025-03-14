mod config;

use config::Config;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    // Load the application configuration
    let cfg = Config::new();

    // Initialize a connection to the SurrealDB server
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    // Authenticate using credentials from our database configuration
    db.signin(Root {
        username: &cfg
            .database
            .username,
        password: &cfg
            .database
            .password,
    })
    .await?;

    // Select the appropriate namespace and database from config
    db.use_ns(
        &cfg.database
            .namespace,
    )
    .use_db(&cfg.database.dbname)
    .await?;

    // Perform some queries
    let some_queries = db
        .query(
            "
            RETURN 9; 
            RETURN 10; 
            SELECT * FROM { is: 'Nice database' };
            ",
        )
        .await?;

    dbg!(some_queries);

    Ok(())
}
