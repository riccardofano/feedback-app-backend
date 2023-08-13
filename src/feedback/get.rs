use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::schema::fetch_request_with_comments;
use crate::Context;

#[debug_handler]
pub async fn get_request(Path(id): Path<i32>, State(context): State<Context>) -> impl IntoResponse {
    match fetch_request_with_comments(&context.pool, id).await {
        Ok(request) => match request {
            Some(r) => (StatusCode::OK, Json(Some(r))),
            None => (StatusCode::NOT_FOUND, Json(None)),
        },
        Err(_error) => (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    }
}
