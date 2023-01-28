use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::SharedState;

pub async fn get_request(
    Path(id): Path<usize>,
    State(state): State<SharedState>,
) -> impl IntoResponse {
    let state = state.read().unwrap();
    let request = state.get_request(id);

    match request {
        Ok(r) => (StatusCode::OK, Json(Some(r.clone()))),
        Err(_) => (StatusCode::NOT_FOUND, Json(None)),
    }
}
