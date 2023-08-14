use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use validator::Validate;

use crate::{
    schema::{Comment, CommentWithReplies},
    Context,
};
use crate::{validation::ValidatedForm, Result};

#[derive(Validate, Serialize, Deserialize)]
pub struct CreateCommentForm {
    username: String,
    #[validate(length(min = 1, max = 250))]
    content: String,
}

pub async fn create_comment(
    Path(request_id): Path<i32>,
    State(context): State<Context>,
    ValidatedForm(form): ValidatedForm<CreateCommentForm>,
) -> Result<impl IntoResponse> {
    let new_comment = insert_comment(&context.pool, request_id, form).await?;
    Ok((StatusCode::CREATED, Json(new_comment)))
}

async fn insert_comment(
    pool: &PgPool,
    id_request: i32,
    form: CreateCommentForm,
) -> anyhow::Result<CommentWithReplies> {
    let comment = sqlx::query_as!(
        Comment,
        "INSERT INTO Comment (id_request, id_parent, owner, content) VALUES ($1, NULL, $2, $3) RETURNING *",
        id_request,
        form.username,
        form.content
    )
    .fetch_one(pool)
    .await?;

    Ok(CommentWithReplies {
        comment,
        replies: Vec::new(),
    })
}
