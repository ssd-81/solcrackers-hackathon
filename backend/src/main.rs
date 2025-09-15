use axum::Router;
use std::net::SocketAddr;

mod routes;
mod handlers;
mod db;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // load DATABASE_URL (from env or .env)
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    // create pool
    let pool = db::create_pool(&database_url).await?;

    // 🔹 Run migrations automatically
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;
    let app: Router = routes::router();
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    // Use tokio + hyper directly
    let listener = tokio::net::TcpListener::bind(&addr).await?;
axum::serve(listener, app).await?;

    Ok(())
}
