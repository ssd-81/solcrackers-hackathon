mod routes;
use axum::{
    routing::{get,post},
    Router,
};
async fn dummy()->&'static str{
    "listsss"
}
#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/ev-request", post(dummy)) // User books a charging slot. 
                            .route("/ev-requests",get(dummy)) //List all bookings.
                            .route("/energy-status", get(dummy)) //Return solar/wind/battery/grid availability (dummy or real API).
                            .route("/charging-plan", get(dummy)) // Run optimization engine and return schedule.
                            .route("/analytics", get(dummy)); //Cost savings, renewable usage, CO₂ avoided.
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}



