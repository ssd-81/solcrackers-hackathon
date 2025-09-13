use axum::Router;
use std::net::SocketAddr;

mod routes;
mod handlers;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app: Router = routes::router();
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    // Use tokio + hyper directly
    let listener = tokio::net::TcpListener::bind(&addr).await?;
axum::serve(listener, app).await?;

    Ok(())
}
