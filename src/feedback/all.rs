use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::{schema::fetch_all_requests, Context, CURRENT_USER};

pub async fn get_feedback_requests(State(context): State<Context>) -> impl IntoResponse {
    let requests = match fetch_all_requests(&context.pool).await {
        Ok(request) => request,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!(err.to_string())),
            )
        }
    };

    let result = json!({
        "currentUser": CURRENT_USER,
        "productRequests": requests
    });

    (StatusCode::OK, Json(result))
}
