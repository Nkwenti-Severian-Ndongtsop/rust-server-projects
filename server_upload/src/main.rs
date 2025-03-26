use reqwest::blocking::multipart::{Form, Part};
use reqwest::blocking::Client;
use std::fs::File;
use std::io::{self, Read};

fn main() {
    let server_url = "http://localhost:8000/upload";
    let client = Client::new();
    let mut files: Vec<String> = Vec::new();

    loop {
        let mut response = String::new();
        println!("Do you want to upload a file? (y/n):");
        io::stdin()
            .read_line(&mut response)
            .expect("Failed to read input");

        match response.trim() {
            "n" => break,
            "y" => {
                let mut file_path = String::new();
                println!("Enter the file path:");
                io::stdin()
                    .read_line(&mut file_path)
                    .expect("Failed to read file path");
                let trimmed_path = file_path.trim().to_string();

                if trimmed_path.is_empty() {
                    println!("File path cannot be empty!");
                } else {
                    files.push(trimmed_path);
                }
            }
            _ => println!("Invalid input. Please enter 'y' or 'n'."),
        }
    }

    if files.is_empty() {
        println!("No files selected for upload.");
        return;
    }

    // Create a multipart form to send all files in one request
    let mut form = Form::new();
    for file_path in &files {
        match File::open(file_path) {
            Ok(mut file) => {
                let mut buffer = Vec::new();
                if file.read_to_end(&mut buffer).is_ok() {
                    let filename = file_path.split('/').last().unwrap_or("unknown_file"); // Extract filename from path
                    let part = Part::bytes(buffer).file_name(filename.to_string());
                    form = form.part("files", part); // Append file to multipart form
                } else {
                    println!("Failed to read file: {}", file_path);
                }
            }
            Err(_) => println!("Could not open file: {}", file_path),
        }
    }

    match client.post(server_url).multipart(form).send() {
        Ok(response) => match response.text() {
            Ok(text) => println!("Server Response: {}", text),
            Err(_) => println!("Failed to read server response."),
        },
        Err(err) => println!("Request failed: {:?}", err),
    }
}
