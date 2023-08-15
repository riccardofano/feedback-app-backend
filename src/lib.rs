mod comment;
mod feedback;
mod schema;
mod validation;

use axum::http::{Method, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, patch, post};
use axum::Router;

use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use comment::{create_comment, create_reply};
use feedback::{create_request, edit_request, get_feedback_requests, get_request, upvote_request};

const CURRENT_USER: &str = "velvetround";
#[derive(Debug, Clone)]
pub struct Context {
    pool: PgPool,
}
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

pub async fn run(pool: PgPool) -> shuttle_axum::ShuttleAxum {
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
        .route("/feedback/:id/upvote", post(upvote_request))
        .route("/feedback/:id/comment", post(create_comment))
        .route(
            "/feedback/:id/comment/:comment_id/reply",
            post(create_reply),
        )
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
