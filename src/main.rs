mod feedback;
mod request;

use std::net::SocketAddr;

use axum::{
    routing::{get, post},
    Router,
};

use feedback::create_request;

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
