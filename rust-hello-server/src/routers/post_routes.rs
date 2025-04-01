use std::path::PathBuf;
use std::{env, fs};
use std::{fs::File, io::Write, path::Path};
use serde_json::json;

use axum::Json;
use axum::{extract::Multipart, response::IntoResponse};
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::{BufReader, BufWriter, Read};

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

    let path: PathBuf = current_dir.join("uploaded_files");

    if let Err(e) = fs::create_dir_all(&path) {
        eprintln!("Error creating directory {}: {}", path.display(), e);
        return "Failed to create uploads directory";
    }

    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        if let Some(file_name) = field.file_name() {
            let file_name = file_name.to_string();
            let data = field.bytes().await.unwrap_or_else(|_| {
                eprintln!("Failed to read file bytes");
                vec![].into()
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


pub async fn compress_file(mut multipart: Multipart) -> impl IntoResponse {
    let output_dir = "compressed_files";
    if !Path::new(output_dir).exists() {
        std::fs::create_dir(output_dir).expect("Failed to create output directory");
    }

    let mut compression_method = "default".to_string();
    let mut responses = Vec::new(); 

    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        if let Some(name) = field.name() {
            if name == "method" {
                compression_method = field.text().await.expect("Invalid compression method");
                continue;
            }
        }

        if let Some(file_name) = field.file_name() {
            let file_name = file_name.to_string();
            let input_path = format!("{}/{}", output_dir, file_name);
            let output_path = format!("{}/{}.gz", output_dir, file_name);

            if let Ok(data) = field.bytes().await {
                if let Err(e) = std::fs::write(&input_path, &data) {
                    eprintln!("Failed to save file {}: {}", input_path, e);
                    responses.push(json!({ "file": file_name, "error": "Failed to save uploaded file" }));
                    continue;
                }
            } else {
                responses.push(json!({ "file": file_name, "error": "Failed to read uploaded file" }));
                continue;
            }

            if let Err(e) = compress_file_internal(&input_path, &output_path, &compression_method) {
                eprintln!("Failed to compress file {}: {}", input_path, e);
                responses.push(json!({ "file": file_name, "error": "Failed to compress file" }));
                continue;
            }       

            responses.push(json!({
                file_name: "successful"
            }));
        }
    }

    if responses.is_empty() {
        Json(json!({ "error": "No valid file uploaded!" }))
    } else {
        Json(json!({ "results": responses }))
    }
}

fn compress_file_internal(input_path: &str, output_path: &str, method: &str) -> std::io::Result<()> {
    let input_file = File::open(input_path)?;
    let mut reader = BufReader::new(input_file);

    let output_file = File::create(output_path)?;
    let writer = BufWriter::new(output_file);

    let compression_level = match method {
        "best" => Compression::best(),
        "fast" => Compression::fast(),
        _ => Compression::default(),
    };

    let mut encoder = GzEncoder::new(writer, compression_level);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    encoder.write_all(&buffer)?;
    encoder.finish()?;

    Ok(())
}

