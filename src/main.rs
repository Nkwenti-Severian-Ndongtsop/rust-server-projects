use std::{fs::File, io::Write, path::Path};

use axum::{
    extract::Multipart,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tokio::net::TcpListener;

async fn upload(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.ok().flatten() {
        if field.name().unwrap_or("") != "fileupload" {
            continue;
        }

        // Get file name
        if let Some(file_name) = field.file_name().map(String::from) {
            let file_path = format!("files/{}", file_name);

            // Ensure the directory exists
            let path = Path::new("files");
            if !path.exists() {
                std::fs::create_dir(path).expect("Failed to create 'files' directory!");
            }

            // Read file bytes
            if let Ok(data) = field.bytes().await {
                // Save the file
                if let Ok(mut file_handle) = File::create(&file_path) {
                    if file_handle.write_all(&data).is_ok() {
                        return format!("File '{}' uploaded successfully!", file_name);
                    }
                }
            }
        }
    }

    "No valid file uploaded!".to_string()
}

fn router() -> Router {
    Router::new()
        .route("/hello", get(display))
        .route("/", get(index).post(upload))
}

async fn display() -> &'static str {
    "Hello, World!"
}

async fn index() -> Html<&'static str> {
    Html(std::include_str!("../public/index.html"))
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:5555";

    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Server running on: http://{}", addr);

    axum::serve(listener, router()).await.unwrap();
}
