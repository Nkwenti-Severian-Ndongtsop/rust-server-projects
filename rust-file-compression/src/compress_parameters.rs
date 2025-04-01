use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn param() -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://halamadrid:mysecretpassword@localhost:5432/server_db")
        .await
        .unwrap()
}

