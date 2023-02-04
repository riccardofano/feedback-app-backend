use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{validation::ValidatedForm, SharedState};

#[derive(Validate, Serialize, Deserialize)]
pub struct CreateRequest {
    #[validate(length(min = 3, message = "Must be at least 3 characters long"))]
    title: String,
    category: String,
    #[validate(length(min = 10, message = "Must be at least 10 characters long"))]
    description: String,
}

pub async fn create_request(
    State(state): State<SharedState>,
    ValidatedForm(payload): ValidatedForm<CreateRequest>,
) -> impl IntoResponse {
    let new_request =
        state
            .write()
            .unwrap()
            .new_request(payload.title, payload.category, payload.description);

    (StatusCode::CREATED, Json(new_request))
}
