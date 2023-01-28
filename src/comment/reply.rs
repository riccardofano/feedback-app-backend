use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Form, Json,
};
use serde::{Deserialize, Serialize};

use crate::SharedState;

#[derive(Serialize, Deserialize)]
pub struct CreateComment {
    username: String,
    to: String,
    content: String,
}

pub async fn create_reply(
    Path(comment_id): Path<usize>,
    State(state): State<SharedState>,
    Form(payload): Form<CreateComment>,
) -> impl IntoResponse {
    let mut state = state.write().unwrap();
    let new_comment = state.new_reply(comment_id, payload.username, payload.content, payload.to);

    (StatusCode::CREATED, Json(new_comment))
}
