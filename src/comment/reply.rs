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
    validation::ValidatedForm,
    {Context, Result},
};

#[derive(Validate, Serialize, Deserialize)]
pub struct CreateReplyForm {
    username: String,
    #[validate(length(min = 1, max = 250))]
    content: String,
}

pub async fn create_reply(
    Path((id_request, id_parent)): Path<(i32, i32)>,
    State(context): State<Context>,
    ValidatedForm(form): ValidatedForm<CreateReplyForm>,
) -> Result<impl IntoResponse> {
    let new_reply = insert_reply(&context.pool, id_request, id_parent, form).await?;

    Ok((StatusCode::CREATED, Json(new_reply)))
}

async fn insert_reply(
    pool: &PgPool,
    id_request: i32,
    id_parent: i32,
    form: CreateReplyForm,
) -> anyhow::Result<CommentWithReplies> {
    let comment = sqlx::query_as!(
        Comment,
        r#"INSERT INTO Comment
        (id_request, id_parent, owner, content)
        VALUES ($1, $2, $3, $4)
        RETURNING *"#,
        id_request,
        id_parent,
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
