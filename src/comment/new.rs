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

    match &new_comment {
        Some(_) => Ok((StatusCode::CREATED, Json(new_comment))),
        None => Ok((StatusCode::BAD_REQUEST, Json(new_comment))),
    }
}

async fn insert_comment(
    pool: &PgPool,
    id_request: i32,
    form: CreateCommentForm,
) -> anyhow::Result<Option<CommentWithReplies>> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM Account WHERE username = $1",
        form.username
    )
    .fetch_optional(pool)
    .await?;

    let Some(user) = user else {
        return Ok(None);
    };

    let comment = sqlx::query_as!(
        Comment,
        r#"INSERT INTO Comment (id_request, id_parent, owner, content)
        VALUES ($1, NULL, $2, $3)
        RETURNING *"#,
        id_request,
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
