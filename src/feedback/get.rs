use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::schema::fetch_request_with_comments;
use crate::{Context, Result};

#[debug_handler]
pub async fn get_request(
    Path(id): Path<i32>,
    State(context): State<Context>,
) -> Result<impl IntoResponse> {
    match fetch_request_with_comments(&context.pool, id).await? {
        Some(r) => Ok((StatusCode::OK, Json(Some(r)))),
        None => Ok((StatusCode::NOT_FOUND, Json(None))),
    }
}
