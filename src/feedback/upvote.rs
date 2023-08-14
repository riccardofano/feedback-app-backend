use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use sqlx::PgPool;

use crate::{Context, Result};

pub async fn upvote_request(
    Path(id_request): Path<i32>,
    State(context): State<Context>,
) -> Result<impl IntoResponse> {
    let upvote_data = update_upvotes(&context.pool, id_request).await?;
    Ok(Json(upvote_data))
}

#[derive(Serialize)]
struct UpvoteData {
    upvotes: i32,
    upvoted: bool,
}

async fn update_upvotes(pool: &PgPool, id_request: i32) -> anyhow::Result<UpvoteData> {
    let row = sqlx::query_as!(
        UpvoteData,
        r#"UPDATE Request SET
        upvotes = CASE WHEN upvoted THEN upvotes - 1 ELSE upvotes + 1 END,
        upvoted = NOT upvoted
        WHERE id = $1
        RETURNING upvotes, upvoted"#,
        id_request
    )
    .fetch_one(pool)
    .await?;

    Ok(row)
}
