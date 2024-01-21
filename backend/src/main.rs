use crate::reservations::Reservations;
use crate::state::AppState;
use axum::{extract::State, response::IntoResponse, routing::get, Router};
use hyper::StatusCode;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod reservation;
pub mod reservations;
pub mod state;

async fn hello(State(state): State<AppState>) -> impl IntoResponse {
    (StatusCode::OK, "Hello world")
}

#[tokio::main]
async fn main() {
    let reservations = Reservations::new_from_file("datafile.csv");
    println!("{:?}", reservations);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_todos=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = AppState::new();

    // Compose the routes
    let app = Router::new()
        .route("/hello", get(hello))
        // Add middleware to all routes
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:6942")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
