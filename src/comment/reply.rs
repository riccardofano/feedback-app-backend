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
    schema::{Comment, CommentWithReplies, CommentWithUser, User},
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

    match &new_reply {
        Some(_) => Ok((StatusCode::CREATED, Json(new_reply))),
        None => Ok((StatusCode::BAD_REQUEST, Json(new_reply))),
    }
}

async fn insert_reply(
    pool: &PgPool,
    id_request: i32,
    id_parent: i32,
    form: CreateReplyForm,
) -> anyhow::Result<Option<CommentWithReplies>> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM Account WHERE username = $1",
        form.username
    )
    .fetch_optional(pool)
    .await?;

    let Some(user) = user else {
        return Ok(None)
    };

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

    let complete_comment = CommentWithUser {
        id: comment.id,
        id_parent: comment.id_parent,
        id_request: comment.id_request,
        content: comment.content,
        user,
    };

    Ok(Some(CommentWithReplies {
        comment: complete_comment,
        replies: Vec::new(),
    }))
}
