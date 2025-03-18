use std::env;
use std::{fs::File, io::Write, path::Path};

use axum::{extract::Multipart, response::IntoResponse};

pub async fn upload(mut multipart: Multipart) -> impl IntoResponse {
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

pub async fn send_file(mut multipart: Multipart) -> &'static str {
    let current_dir = env::current_dir().unwrap();

    let path = current_dir.join("uploads"); // Use absolute path

    if !path.exists() {
        std::fs::create_dir(&path).expect("Failed to create 'uploads' directory!");
    }

    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap_or("uploaded_file").to_string();
        let data = field.bytes().await.unwrap();

        let file_path = path.join(&file_name);
        println!("Saving file to: {}", file_path.display());

        let mut file = File::create(&file_path).unwrap();
        file.write_all(&data).unwrap();
    }
    "File uploaded successfully!"
}
