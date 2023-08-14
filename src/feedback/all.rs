use axum::{extract::State, response::IntoResponse, Json};
use serde_json::json;

use crate::{schema::fetch_all_requests, Context, Result, CURRENT_USER};

pub async fn get_feedback_requests(State(context): State<Context>) -> Result<impl IntoResponse> {
    let requests = fetch_all_requests(&context.pool).await?;

    let result = json!({
        "currentUser": CURRENT_USER,
        "productRequests": requests
    });

    Ok(Json(result))
}
