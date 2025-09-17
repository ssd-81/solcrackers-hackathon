use axum::Router;
use std::net::SocketAddr;
mod db;
mod handlers;
mod routes;
use dotenvy::dotenv;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
}
use std::sync::Arc;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // load DATABASE_URL (from env or .env)
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create pool
    let pool = db::create_pool(&database_url).await?;
    let shared_state = Arc::new(AppState { db_pool: pool.clone() });
    // 🔹 Run migrations automatically
    sqlx::migrate!("./migrations").run(&pool).await?;
    let app: Router = routes::router(shared_state.clone());
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    // Use tokio + hyper directly
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
