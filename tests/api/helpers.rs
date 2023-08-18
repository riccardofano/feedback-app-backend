use dotenvy_macro::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};

async fn create_pool() -> PgPool {
    let pool = PgPoolOptions::new()
        .connect(dotenv!("DATABASE_URL"))
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
