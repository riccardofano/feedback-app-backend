use dotenvy_macro::dotenv;
use sqlx::PgPool;

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_shared_db::Postgres(
    local_uri = dotenv!("DATABASE_URL")
)]
    pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
    rf_feedback_app::run(pool).await
}
