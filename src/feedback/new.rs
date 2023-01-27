use axum::{extract::State, http::StatusCode, response::IntoResponse, Form, Json};
use serde::{Deserialize, Serialize};

use crate::SharedState;

#[derive(Serialize, Deserialize)]
pub struct CreateRequest {
    title: String,
    category: String,
    description: String,
}

pub async fn create_request(
    State(state): State<SharedState>,
    Form(payload): Form<CreateRequest>,
) -> impl IntoResponse {
    let new_request =
        state
            .write()
            .unwrap()
            .new_request(payload.title, payload.category, payload.description);

    (StatusCode::CREATED, Json(new_request))
}
