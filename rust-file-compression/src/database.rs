pub async fn insert_user(pool1: &sqlx::PgPool, pool2: &sqlx::PgPool, file_path: &str) -> u64 {
    let compressed_file = format!("{}.gz", file_path);

    let query = "INSERT INTO files (file_path, compressed_file) VALUES ($1, $2)";
    sqlx::query(query)
        .bind(file_path)
        .bind(compressed_file)
        .execute(pool1)
        .await
        .unwrap();

    let result = sqlx::query("SELECT id FROM files WHERE file_path=$1")
        .bind(file_path)
        .execute(pool2)
        .await
        .unwrap();
    result.rows_affected()
}
