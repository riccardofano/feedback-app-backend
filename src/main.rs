use std::net::SocketAddr;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Form, Json, Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/feedback/new", post(create_request));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::info!("listening on {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("could not start server");
}

async fn root() -> &'static str {
    "Hello world"
}

#[derive(Serialize, Deserialize)]
struct CreateRequest {
    title: String,
    category: String,
    description: String,
}

#[derive(Serialize, Deserialize)]
struct Request {
    id: u32,
    title: String,
    category: String,
    upvotes: u32,
    status: String,
    description: String,
    comments: Option<String>,
}

async fn create_request(Form(payload): Form<CreateRequest>) -> impl IntoResponse {
    let new_request = Request {
        id: 1,
        title: payload.title,
        category: payload.category,
        upvotes: 0,
        status: String::from("suggestion"),
        description: payload.description,
        comments: None,
    };

    (StatusCode::CREATED, Json(new_request))
}
