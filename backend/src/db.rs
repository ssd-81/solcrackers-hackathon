// db.rs

use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;
use anyhow::Result;

/// Create a connection pool to be shared across your application.
///
/// # Arguments
///
/// * `database_url` - A PostgreSQL database connection string.
///
/// # Returns
///
/// * `Result<PgPool, anyhow::Error>` - The connection pool if successful, or an error.
pub async fn create_pool(database_url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_timeout(Duration::from_secs(5))
        .connect(database_url)
        .await?;

    Ok(pool)
}
