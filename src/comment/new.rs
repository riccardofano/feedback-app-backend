use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{validation::ValidatedForm, SharedState};

#[derive(Validate, Serialize, Deserialize)]
pub struct CreateComment {
    username: String,
    #[validate(length(
        min = 1,
        max = 250,
        message = "Content must be between 1 and 250 characters long"
    ))]
    content: String,
}

pub async fn create_comment(
    Path(request_id): Path<usize>,
    State(state): State<SharedState>,
    ValidatedForm(payload): ValidatedForm<CreateComment>,
) -> impl IntoResponse {
    let mut state = state.write().unwrap();
    let new_comment = state.new_comment(request_id, payload.username, payload.content);

    (StatusCode::CREATED, Json(new_comment))
}
