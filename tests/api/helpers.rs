use hyper::{http::HeaderValue, Body, HeaderMap, Request, Response};
use serde::de::DeserializeOwned;
use sqlx::{postgres::PgPoolOptions, PgPool};

const TEST_DATABASE_URL: &str = "postgres://postgres:postgres@localhost:23871/postgres";

async fn create_pool() -> PgPool {
    let pool = PgPoolOptions::new()
        .connect(TEST_DATABASE_URL)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to migrate the database");
    pool
}

pub async fn create_app() -> axum::Router {
    let pool = create_pool().await;

    let app = rf_feedback_app::run(pool)
        .await
        .expect("Failed to setup the app from library");

    app.0
}

pub fn is_application_json(headers: &HeaderMap<HeaderValue>) -> bool {
    headers.get("Content-Type") == Some(&HeaderValue::from_str("application/json").unwrap())
}

pub async fn parse_response_body<T, B>(response: Response<B>) -> T
where
    T: DeserializeOwned,
    B: http_body::Body,
    <B as http_body::Body>::Error: std::fmt::Debug,
{
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    serde_json::from_slice(&body).unwrap()
}

pub async fn get_request(uri: &str, body: Body) -> Request<Body> {
    Request::builder().uri(uri).body(body).unwrap()
}

pub async fn post_request(uri: &str, body: Body) -> Request<Body> {
    Request::builder()
        .method("POST")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .uri(uri)
        .body(body)
        .unwrap()
}

pub async fn patch_request(uri: &str, body: Body) -> Request<Body> {
    Request::builder()
        .method("PATCH")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .uri(uri)
        .body(body)
        .unwrap()
}
