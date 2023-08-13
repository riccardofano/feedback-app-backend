use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::schema::{Comment, Request};
use crate::Context;

#[debug_handler]
pub async fn get_request(Path(id): Path<i32>, State(context): State<Context>) -> impl IntoResponse {
    let row = sqlx::query_as!(
        Request,
        r#"SELECT r.*,
        ARRAY_AGG((
            c.id,
            c.id_request,
            c.id_parent,
            c.owner,
            c.content
        )) as "comments: Vec<Comment>"
        FROM Request r
        LEFT JOIN Comment c
            ON r.id = c.id_request AND c.id_parent IS NULL
        WHERE r.id = $1
        GROUP BY r.id"#,
        id
    )
    .fetch_optional(&context.pool)
    .await
    .expect("Could not fetch request");

    match row {
        Some(request) => (StatusCode::OK, Json(Some(request))),
        None => (StatusCode::NOT_FOUND, Json(None)),
    }
}
