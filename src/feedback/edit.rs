use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use validator::Validate;

use crate::{validation::ValidatedForm, SharedState};

#[derive(Validate, Deserialize)]
pub struct EditForm {
    #[validate(length(min = 3))]
    title: String,
    category: String,
    status: String,
    #[validate(length(min = 10))]
    description: String,
}

pub async fn edit_request(
    Path(id): Path<usize>,
    State(state): State<SharedState>,
    ValidatedForm(form): ValidatedForm<EditForm>,
) -> impl IntoResponse {
    let mut state = state.write().unwrap();
    let request = state.edit_request(id, form.title, form.category, form.status, form.description);

    match request {
        Ok(r) => (StatusCode::OK, Json(Some(r))),
        Err(_) => (StatusCode::NOT_FOUND, Json(None)),
    }
}
