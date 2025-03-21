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
    let args = Args::parse();
    let server_url = "http://localhost:8000/upload";

    // Read the ca
    for file_path in &args.file_path {
        let mut file = File::open(file_path).expect("Failed to open file");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("Failed to read file");

        // Create a multipart form
        let part = Part::bytes(buffer).file_name(file_path.clone());
        let form = Form::new().part("file", part);

        // Send the file
        let client = Client::new();
        let response = client
            .post(server_url)
            .multipart(form)
            .send()
            .expect("Failed to send request");
        println!("Server Response: {:?}", response.text().unwrap());
    }
}
