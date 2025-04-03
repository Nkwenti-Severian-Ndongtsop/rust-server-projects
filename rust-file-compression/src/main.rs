use clap::{Arg, Command};
use dotenvy::dotenv;
use reqwest::multipart::{Form, Part};
use reqwest::Client;
use std::env;
use std::fs::File;
use std::io::{self, Read};
mod database;

use database::insert_user;
use database::update_state;
use database::FileStatus;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let matches = Command::new("Compress")
        .version("1.0")
        .author("Nkwenti-Severian")
        .about("Uploads and compresses files with specified method")
        .arg(
            Arg::new("compress")
                .short('c')
                .long("compress")
                .value_name("FILES")
                .num_args(1..)
                .help("List of files to compress")
                .required(true),
        )
        .arg(
            Arg::new("method")
                .short('m')
                .long("method")
                .value_name("METHOD")
                .help("Compression method: best, fast, default")
                .default_value("default"),
        )
        .get_matches();

    let files: Vec<&str> = matches
        .get_many::<String>("compress")
        .unwrap()
        .map(|s| s.as_str())
        .collect();

    let method = matches.get_one::<String>("method").unwrap();
    let server_url = "http://127.0.0.1:8000/compress";
    let client = Client::new();

    match compress_and_upload(&files, method, &client, server_url).await {
        Ok(response) => {
            let database_url = match env::var("DATABASE_URL") {
                Ok(url) => url,
                Err(e) => {
                    eprintln!("Failed to read DATABASE_URL: {}", e);
                    return;
                }
            };
            let pool = sqlx::postgres::PgPool::connect(&database_url).await.unwrap();

            println!("\nServer Response: {}\n", response);

            for file in files {
                let id = insert_user(&pool, file).await;
                let _ = update_state(&pool, id as i32, FileStatus::Completed).await;
                println!(
                    "The file {}\n\nHas ID: {}\n\nFile State: completed",
                    file, id,
                )
            }
        }
        Err(e) => {
            let database_url = match env::var("DATABASE_URL") {
                Ok(url) => url,
                Err(e) => {
                    eprintln!("Failed to read DATABASE_URL: {}", e);
                    return;
                }
            };
            let pool = sqlx::postgres::PgPool::connect(&database_url).await.unwrap();
            for file in files {
                let id = insert_user(&pool, file).await;
                let _ = update_state(&pool, id as i32, FileStatus::Failed).await;
                println!("The file {}\n\nHas ID: {}\n\nFile State: failed", file, id,)
            }
            eprintln!("Failed to upload files: {}\n", e)
        }
    }
}

async fn compress_and_upload(
    files: &[&str],
    method: &str,
    client: &Client,
    server_url: &str,
) -> Result<String, reqwest::Error> {
    let mut form = Form::new();

    for file_path in files {
        if let Ok(file_bytes) = read_file(file_path) {
            let filename = file_path.split('/').last().unwrap_or("unknown_file");
            form = form.part(
                "files",
                Part::bytes(file_bytes).file_name(filename.to_string()),
            );
        } else {
            eprintln!("Failed to read file: {}\n", file_path);
        }
    }

    form = form.text("method", method.to_string());

    let response = client.post(server_url).multipart(form).send().await?;
    let response_text = response.text().await?;

    Ok(response_text)
}

fn read_file(file_path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
