use sqlx::{PgPool, Pool, Postgres};
use anyhow::Result;

pub type DbPool = Pool<Postgres>;

pub async fn create_pool(database_url: &str) -> Result<PgPool> {
    println!("Connecting to database: {}", database_url);
    
    let pool = PgPool::connect(database_url).await?;
    
    println!("Database connection established");
    
    Ok(pool)
}

pub async fn test_connection(pool: &DbPool) -> Result<()> {
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(pool)
        .await?;

    println!("Database test query successful: {}", row.0);
    
    Ok(())
}