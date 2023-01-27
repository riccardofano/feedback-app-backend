use axum::{http::StatusCode, response::IntoResponse, Form, Json};
use serde::{Deserialize, Serialize};

use crate::request::Request;

#[derive(Serialize, Deserialize)]
pub struct CreateRequest {
    title: String,
    category: String,
    description: String,
}

pub async fn create_request(Form(payload): Form<CreateRequest>) -> impl IntoResponse {
    let new_request = Request::new(payload.title, payload.category, payload.description);
    (StatusCode::CREATED, Json(new_request))
}
