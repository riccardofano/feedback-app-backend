use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use sqlx::PgPool;
use validator::Validate;

use crate::{schema::Request, validation::ValidatedForm, Context, Result};

#[derive(Debug, Validate, Deserialize)]
pub struct EditForm {
    #[validate(length(min = 3))]
    pub title: String,
    pub category: String,
    pub status: String,
    #[validate(length(min = 10))]
    pub description: String,
}

#[tracing::instrument]
pub async fn edit_request(
    Path(id): Path<i32>,
    State(context): State<Context>,
    ValidatedForm(form): ValidatedForm<EditForm>,
) -> Response {
    let request = match update_request(&context.pool, id, form).await {
        Ok(request) => request,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    (StatusCode::OK, Json(request)).into_response()
}

async fn update_request(pool: &PgPool, id_request: i32, form: EditForm) -> Result<Request> {
    let row = sqlx::query_as!(
        Request,
        "UPDATE Request SET title = $1, category = $2, status = $3, description = $4 WHERE id = $5 RETURNING *",
        form.title,
        form.category,
        form.status,
        form.description,
        id_request
    )
    .fetch_one(pool)
    .await?;

    Ok(row)
}
