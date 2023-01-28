mod appstate;
mod comment;
mod feedback;

use std::{
    net::SocketAddr,
    sync::{Arc, RwLock},
};

use axum::{
    routing::{get, patch, post},
    Router,
};

use feedback::create_request;

use crate::{
    appstate::AppState,
    comment::{create_comment, create_reply},
    feedback::{edit_request, get_feedback_requests, get_request, upvote_request},
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
        .route("/feedback/:id/edit", patch(edit_request))
        .route("/feedback/:id/upvote", post(upvote_request))
        .route("/feedback/:id/comment", post(create_comment))
        .route("/comments/:id/reply", post(create_reply))
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
