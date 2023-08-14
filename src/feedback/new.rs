use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use validator::Validate;

use crate::{schema::Request, validation::ValidatedForm, Context, Result};

#[derive(Validate, Serialize, Deserialize)]
pub struct NewFeedbackForm {
    #[validate(length(min = 3))]
    title: String,
    category: String,
    status: String,
    #[validate(length(min = 10))]
    description: String,
}

pub async fn create_request(
    State(context): State<Context>,
    ValidatedForm(form): ValidatedForm<NewFeedbackForm>,
) -> Result<impl IntoResponse> {
    let feedback = insert_feedback(&context.pool, form).await?;

    Ok((StatusCode::CREATED, Json(feedback)))
}

async fn insert_feedback(pool: &PgPool, form: NewFeedbackForm) -> anyhow::Result<Request> {
    let row = sqlx::query_as!(
        Request,
        "INSERT INTO Request (title, category, status, description) VALUES ($1, $2, $3, $4) RETURNING *",
        form.title,
        form.category,
        form.status,
        form.description
    )
    .fetch_one(pool)
    .await?;

    Ok(row)
}
