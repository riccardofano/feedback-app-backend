mod appstate;
mod feedback;

use std::{
    net::SocketAddr,
    sync::{Arc, RwLock},
};

use axum::{
    routing::{get, post},
    Router,
};

use feedback::create_request;

use crate::{
    appstate::AppState,
    feedback::{get_feedback_requests, get_request},
};

type SharedState = Arc<RwLock<AppState>>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = Arc::new(RwLock::new(AppState::seeded()));

    let app = Router::new()
        .route("/", get(root))
        .route("/feedback/all", get(get_feedback_requests))
        .route("/feedback/new", post(create_request))
        .route("/feedback/:id", get(get_request))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    tracing::info!("listening on {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("could not start server");
}

async fn root() -> &'static str {
    "Hello world"
}
