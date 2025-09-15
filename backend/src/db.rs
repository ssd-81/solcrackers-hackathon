use sqlx::{postgres::PgPoolOptions, PgPool};
use anyhow::Result;

/// Create a connection pool to be shared across your application.
pub async fn create_pool(database_url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    Ok(pool)
}

