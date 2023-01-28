use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::SharedState;

pub async fn upvote_request(
    Path(id): Path<usize>,
    State(state): State<SharedState>,
) -> impl IntoResponse {
    let mut state = state.write().unwrap();
    let request = state.upvote_request(id);

    match request {
        Ok(r) => (
            StatusCode::OK,
            Json(Some(json!(
            {
                "upvoted": r.upvoted,
                "upvotes": r.upvotes
            }))),
        ),
        Err(_) => (StatusCode::NOT_FOUND, Json(None)),
    }
}
