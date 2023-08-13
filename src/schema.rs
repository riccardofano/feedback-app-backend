use anyhow::Result;
use axum::http::request;
use serde::Serialize;
use sqlx::{Acquire, FromRow, PgConnection, PgPool};

#[derive(Debug, FromRow, Serialize)]
pub struct Request {
    pub id: i32,
    pub title: String,
    pub category: String,
    pub upvotes: i32,
    pub upvoted: bool,
    pub status: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct RequestWithComments {
    #[serde(flatten)]
    pub request: Request,
    pub comments: Vec<CommentWithReplies>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct Comment {
    pub id: i32,
    pub id_request: i32,
    pub id_parent: Option<i32>,
    pub owner: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct CommentWithReplies {
    #[serde(flatten)]
    pub comment: Comment,
    pub replies: Vec<CommentWithReplies>,
}

pub async fn fetch_request_with_comments(
    pool: &PgPool,
    id_request: i32,
) -> Result<Option<RequestWithComments>> {
    let request_row = sqlx::query_as!(Request, "SELECT * FROM Request WHERE id = $1", id_request)
        .fetch_optional(pool)
        .await?;

    let Some(request) = request_row else {
        return Ok(None)
    };

    let comments = sqlx::query_as!(
        Comment,
        "SELECT * FROM Comment WHERE id_request = $1",
        id_request
    )
    .fetch_all(pool)
    .await?;

    let mut nested_comments: Vec<CommentWithReplies> = Vec::new();
    for comment in comments {
        let parent_id = comment.id_parent;
        let new_comment = CommentWithReplies {
            comment,
            replies: Vec::new(),
        };

        if let Some(id) = parent_id {
            let Some(parent_comment) = nested_comments.iter_mut().find(|c| c.comment.id == id) else {
                continue;
            };
            parent_comment.replies.push(new_comment)
        } else {
            nested_comments.push(new_comment)
        }
    }

    Ok(Some(RequestWithComments {
        request,
        comments: nested_comments,
    }))
}
