use std::collections::HashMap;

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
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: i32,
    pub id_request: i32,
    pub id_parent: Option<i32>,
    pub content: String,
    pub owner: String,
    pub replying_to: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentWithUser {
    pub id: i32,
    pub id_request: i32,
    pub id_parent: Option<i32>,
    pub content: String,
    pub replying_to: Option<String>,
    pub user: User,
}

#[derive(Serialize)]
pub struct CommentWithReplies {
    #[serde(flatten)]
    pub comment: CommentWithUser,
    pub replies: Vec<CommentWithReplies>,
}

#[derive(Debug, sqlx::Type, Serialize)]
pub struct User {
    pub image: String,
    pub name: String,
    pub username: String,
}

pub async fn fetch_all_requests(pool: &PgPool) -> anyhow::Result<Vec<Request>> {
    let rows = sqlx::query_as!(Request, "SELECT * FROM Request")
        .fetch_all(pool)
        .await?;

    Ok(rows)
}

pub async fn fetch_request(pool: &PgPool, id_request: i32) -> anyhow::Result<Option<Request>> {
    let row = sqlx::query_as!(Request, "SELECT * FROM Request WHERE id = $1", id_request)
        .fetch_optional(pool)
        .await?;

    Ok(row)
}

pub async fn fetch_request_with_comments(
    pool: &PgPool,
    id_request: i32,
) -> anyhow::Result<Option<RequestWithComments>> {
    let Some(request) = fetch_request(pool, id_request).await? else {
        return Ok(None)
    };

    // NOTE: I'd like to fetch and nest all comments in one go but it seems
    // tricky with sqlx and I think it's fine for now since it's just a demo
    let comment_rows = sqlx::query_as!(
        CommentWithUser,
        // NOTE: You can't use `u.*`, you have to specify the names explicitely
        r#"SELECT
        c.id, c.id_parent, c.id_request, c.content,
        (u.image, u.name, u.username) as "user!: User",
        parent.owner as "replying_to: Option<String>"
        FROM Comment c
        INNER JOIN Account u
        ON c.owner = u.username
        LEFT OUTER JOIN Comment parent
        ON c.id_parent = parent.id
        WHERE c.id_request = $1
        ORDER BY id"#,
        id_request
    )
    .fetch_all(pool)
    .await?;

    let mut comment_map: HashMap<i32, Vec<CommentWithReplies>> = HashMap::new();
    let mut base_comments = Vec::new();

    for comment in comment_rows.into_iter().rev() {
        let child_replies = comment_map.remove_entry(&comment.id);
        let complete_comment = CommentWithReplies {
            comment,
            replies: child_replies.map(|entry| entry.1).unwrap_or_default(),
        };

        if let Some(id_parent) = complete_comment.comment.id_parent {
            let sibling_replies = comment_map.entry(id_parent).or_insert(Vec::new());
            sibling_replies.push(complete_comment);
        } else {
            base_comments.push(complete_comment);
        }
    }

    Ok(Some(RequestWithComments {
        request,
        comments: base_comments.into_iter().rev().collect(),
    }))
}

pub async fn fetch_user(pool: &PgPool, username: &str) -> anyhow::Result<Option<User>> {
    let user = sqlx::query_as!(User, "SELECT * FROM Account WHERE username = $1", username)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}
