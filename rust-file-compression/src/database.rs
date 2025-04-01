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

use sqlx::PgPool;

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "file_status", rename_all = "snake_case")] // Ensure the enum matches the DB's case convention
pub enum FileStatus {
    Pending,
    Completed,
    Failed,
}

pub async fn update_state(
    pool: &PgPool,
    file_id: i32,
    new_status: FileStatus,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE files SET file_state = $1 WHERE id = $2")
        .bind(new_status) // Use `.to_string()` to bind the enum as a string
        .bind(file_id)
        .execute(pool)
        .await?;

    println!("Successfully updated the value for file ID: {}", file_id);

    Ok(())
}
