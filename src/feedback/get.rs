use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::schema::Request;
use crate::Context;

#[debug_handler]
pub async fn get_request(Path(id): Path<i32>, State(context): State<Context>) -> impl IntoResponse {
    let row = sqlx::query_as!(Request, "SELECT * FROM Request WHERE id = $1", id)
        .fetch_optional(&context.pool)
        .await
        .ok()
        .flatten();

    match row {
        Some(request) => (StatusCode::OK, Json(Some(request))),
        None => (StatusCode::NOT_FOUND, Json(None)),
    }
}
