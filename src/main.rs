mod appstate;
mod comment;
mod feedback;
mod schema;
mod validation;

use std::sync::{Arc, RwLock};

use axum::{
    http::{Method, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, patch, post},
    Router,
};

use dotenvy_macro::dotenv;
use feedback::create_request;
use sqlx::PgPool;
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

pub type Result<T> = std::result::Result<T, AppError>;
pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

#[derive(Debug, Clone)]
pub struct Context {
    pool: PgPool,
}
const CURRENT_USER: &str = "velvetround";

type SharedState = Arc<RwLock<AppState>>;

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_shared_db::Postgres(
    local_uri = dotenv!("DATABASE_URL")
)]
    pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PATCH])
        .allow_origin(Any);

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Could not run the db migrations");

    let app = Router::new()
        .route("/", get(root))
        .route("/feedback/all", get(get_feedback_requests))
        .route("/feedback/new", post(create_request))
        .route("/feedback/:id", get(get_request))
        .route("/feedback/:id/edit", patch(edit_request))
        // .route("/feedback/:id/upvote", post(upvote_request))
        .route("/feedback/:id/comment", post(create_comment))
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
