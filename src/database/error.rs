use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Database connection error: {0}")]
    ConnectionError(#[from] surrealdb::Error),
    // #[error("Database configuration error: {0}")]
    // ConfigError(String),
    //
    // #[error("Database query error: {0}")]
    // QueryError(String),
}

// Create a Result type alias for database operations
pub type DatabaseResult<T> = Result<T, DatabaseError>;

