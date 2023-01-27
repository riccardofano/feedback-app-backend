use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::SharedState;

pub async fn get_feedback_requests(State(state): State<SharedState>) -> impl IntoResponse {
    (StatusCode::OK, Json(state.read().unwrap().clone()))
}
