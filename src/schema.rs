use crate::Result;
use serde::Serialize;
use sqlx::{FromRow, PgPool};

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

pub async fn fetch_all_requests(pool: &PgPool) -> Result<Vec<Request>> {
    let rows = sqlx::query_as!(Request, "SELECT * FROM Request")
        .fetch_all(pool)
        .await?;

    Ok(rows)
}

pub async fn fetch_request(pool: &PgPool, id_request: i32) -> Result<Option<Request>> {
    let row = sqlx::query_as!(Request, "SELECT * FROM Request WHERE id = $1", id_request)
        .fetch_optional(pool)
        .await?;

    Ok(row)
}

pub async fn fetch_request_with_comments(
    pool: &PgPool,
    id_request: i32,
) -> Result<Option<RequestWithComments>> {
    let Some(request) = fetch_request(pool, id_request).await? else {
        return Ok(None)
    };

    // NOTE: I'd like to fetch and nest all comments in one go but it seems
    // tricky with sqlx and I think it's fine for now since it's just a demo
    let comments = sqlx::query_as!(
        Comment,
        "SELECT * FROM Comment WHERE id_request = $1 ORDER BY id",
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
