use std::{env, fs};
use std::path::PathBuf;
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
    
    let current_dir = env::current_dir().expect("Failed to get current directory");

    
    let path: PathBuf = current_dir.join("server_upload");

    
    if let Err(e) = fs::create_dir_all(&path) {
        eprintln!("Error creating directory {}: {}", path.display(), e);
        return "Failed to create uploads directory";
    }

    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        if let Some(file_name) = field.file_name() {
            let file_name = file_name.to_string();
            let data = field.bytes().await.unwrap_or_else(|_| {
                eprintln!("Failed to read file bytes");
                return vec![].into();
            });

            let file_path = path.join(&file_name);
            println!("Saving file to: {}", file_path.display());

            match File::create(&file_path) {
                Ok(mut file) => {
                    if let Err(e) = file.write_all(&data) {
                        eprintln!("Error writing file {}: {}", file_path.display(), e);
                        return "Failed to write file";
                    }
                }
                Err(e) => {
                    eprintln!("Failed to create file {}: {}", file_path.display(), e);
                    return "Failed to create file";
                }
            }
        } else {
            eprintln!("File name not found in multipart data");
            return "File name missing";
        }
    }
    "File uploaded successfully!"
}