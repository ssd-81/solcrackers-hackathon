use axum::Router;
use std::net::SocketAddr;
mod db;
mod handlers;
mod routes;
mod models;
mod repository;
use dotenvy::dotenv;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
}

use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logger
    env_logger::init();

    // load DATABASE_URL (from env or .env)
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("Starting EV Charging Backend...");

    // create pool
    let pool = db::create_pool(&database_url).await?;
    
    // Test connection
    db::test_connection(&pool).await?;
    
    let shared_state = Arc::new(AppState { db_pool: pool.clone() });
    
    // 🔹 Run migrations automatically
    println!("Running database migrations...");
    sqlx::migrate!("./migrations").run(&pool).await?;
    println!("Migrations completed successfully");

    let app: Router = routes::router(shared_state.clone());
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    println!("erver starting on http://0.0.0.0:3000");
    println!("API Endpoints:");
    println!("  POST   /api/bookings              - Create new booking");
    println!("  GET    /api/bookings              - Get all bookings");
    println!("  GET    /api/bookings/:slot_id     - Get booking by slot ID");
    println!("  GET    /api/bookings/ev/:ev_id    - Get bookings by EV ID");
    println!("  PUT    /api/bookings/:slot_id/status/:status - Update booking status");

    // Use tokio + hyper directly
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}