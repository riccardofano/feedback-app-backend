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
    to: String,
    #[validate(length(min = 1, max = 250))]
    content: String,
}

pub async fn create_reply(
    Path(comment_id): Path<usize>,
    State(state): State<SharedState>,
    ValidatedForm(payload): ValidatedForm<CreateComment>,
) -> impl IntoResponse {
    let mut state = state.write().unwrap();
    let new_comment = state.new_reply(comment_id, payload.username, payload.content, payload.to);

    (StatusCode::CREATED, Json(new_comment))
}
