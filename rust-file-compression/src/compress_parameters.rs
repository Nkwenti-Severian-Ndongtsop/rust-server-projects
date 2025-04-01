
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn param() -> Pool<Postgres> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://postgres:mysecretpassword@10.39.78.149:5432/upload")
        .await
        .unwrap();
    pool
}