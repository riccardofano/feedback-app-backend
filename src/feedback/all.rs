use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::SharedState;

pub async fn get_feedback_requests(State(state): State<SharedState>) -> impl IntoResponse {
    let state = state.read().unwrap().clone();
    let current_user = state.get_current_user();
    let product_requests = state.get_all_requests();

    (
        StatusCode::OK,
        Json(json!({
            "currentUser": current_user,
            "productRequests": product_requests
        })),
    )
}
