use clap::Parser;
use reqwest::blocking::multipart::{Form, Part};
use reqwest::blocking::Client;
use std::fs::File;
use std::io::Read;

/// Simple file uploader CLI
#[derive(Parser)]
#[command(version = "1.0", about = "Uploads a file to the server")]
struct Args {
    /// Path to the file to upload
    file_path: Vec<String>,
}

fn main() {
    let server_url = "http://localhost:8000/upload";
    let mut files: Vec<String> = Vec::new();

    loop {
        let mut input = String::new();
        println!("Do you want to upload a file (y/n):");
        let mut response = String::new();
        std::io::stdin()
            .read_line(&mut response)
            .expect("Invalid input for file");
        if response.trim() == "n" {
            break;
        } else if response.trim() == "y" {
            println!("Enter the file path:");
            std::io::stdin()
                .read_line(&mut input)
                .expect("Invaliinput input for file");
            files.push(input.trim().to_string());
        }
        println!("{:?}", files)
    }

    for file_path in &files {
        let mut file = File::open(file_path).expect("Failed to open file");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("Failed to read file");

        let part = Part::bytes(buffer).file_name(file_path.clone());
        let form = Form::new().part("file", part);

        let client = Client::new();
        let response = client
            .post(server_url)
            .multipart(form)
            .send()
            .expect("Failed to send request");
        println!("Server Response: {:?}", response.text().unwrap());
    }
}
