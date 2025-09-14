use axum::Router;
use std::net::SocketAddr;

mod routes;
mod db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Use the correct function name
    let pool = db::get_db_pool().await;
    
    let app: Router = routes::router(pool);
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}