mod appstate;
mod comment;
mod feedback;
mod validation;

use std::sync::{Arc, RwLock};

use axum::{
    http::Method,
    routing::{get, patch, post},
    Router,
};

use feedback::create_request;
use sqlx::{Executor, PgPool};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{self, TraceLayer},
};
use tracing::Level;

use crate::{
    appstate::AppState,
    comment::{create_comment, create_reply},
    feedback::{edit_request, get_feedback_requests, get_request, upvote_request},
};

#[derive(Clone)]
struct Context {
    pool: PgPool,
}

type SharedState = Arc<RwLock<AppState>>;

#[shuttle_runtime::main]
async fn axum(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PATCH])
        .allow_origin(Any);

    pool.execute(include_str!("../migrations/1_setup.sql"))
        .await
        .expect("Could not execute database setup schema");

    pool.execute(include_str!("../migrations/2_seed.sql"))
        .await
        .expect("Could not seed the database");

    let app = Router::new()
        .route("/", get(root))
        // .route("/feedback/all", get(get_feedback_requests))
        // .route("/feedback/new", post(create_request))
        // .route("/feedback/:id", get(get_request))
        // .route("/feedback/:id/edit", patch(edit_request))
        // .route("/feedback/:id/upvote", post(upvote_request))
        // .route("/feedback/:id/comment", post(create_comment))
        // .route("/comments/:id/reply", post(create_reply))
        .with_state(Context { pool })
        .layer(cors)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO))
                .on_request(trace::DefaultOnRequest::new().level(Level::INFO))
                .on_failure(trace::DefaultOnFailure::new().level(Level::ERROR)),
        );

    Ok(app.into())
}

async fn root() -> &'static str {
    "Hello world"
}
