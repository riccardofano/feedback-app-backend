use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Form, Json,
};
use serde::Deserialize;

use crate::SharedState;

#[derive(Deserialize)]
pub struct EditForm {
    title: String,
    category: String,
    status: String,
    description: String,
}

pub async fn edit_request(
    Path(id): Path<usize>,
    State(state): State<SharedState>,
    Form(form): Form<EditForm>,
) -> impl IntoResponse {
    let mut state = state.write().unwrap();
    let request = state.edit_request(id, form.title, form.category, form.status, form.description);

    match request {
        Ok(r) => (StatusCode::OK, Json(Some(r))),
        Err(_) => (StatusCode::NOT_FOUND, Json(None)),
    }
}
