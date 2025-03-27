use reqwest::blocking::multipart::{Form, Part};
use reqwest::blocking::Client;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut input_files: Vec<String> = Vec::new();
    let mut comp_meth: Vec<String> = Vec::new();
    let server_url = "http://localhost:8000/compress";

    loop {
        let mut response = String::new();
        let mut i_file = String::new();
        let mut comp_method = String::new();
        println!("Do you want to compress files (y/n)");

        std::io::stdin()
            .read_line(&mut response)
            .expect("Failed to take response");

        if response.trim() == "n" {
            break;
        } else if response.trim() == "y" {
            println!("Enter the file to be compressed:");
            std::io::stdin()
                .read_line(&mut i_file)
                .expect("Failed to take file");
            input_files.push(i_file.trim().to_string());

            println!("Enter compression method (e.g., best, fast, default):");
            std::io::stdin()
                .read_line(&mut comp_method)
                .expect("Failed to take compression method");
            comp_meth.push(comp_method.trim().to_string());
        }
    }

    if input_files.is_empty() {
        println!("No files to compress. Exiting.");
        return;
    }

    let mut form = Form::new();
    for (i, file_path) in input_files.iter().enumerate() {
        let method = &comp_meth[i];
        match File::open(file_path) {
            Ok(mut file) => {
                let mut buffer = Vec::new();
                if file.read_to_end(&mut buffer).is_ok() {
                    let filename = file_path.split('/').last().unwrap_or("unknown_file");
                    let file_part = Part::bytes(buffer).file_name(filename.to_string());
                    form = form.part(format!("file_{}", i), file_part);
                    form = form.text(format!("method_{}", i), method.clone());
                }
            }
            Err(e) => {
                eprintln!("Failed to read file '{}': {}", file_path, e);
                return;
            }
        }
    }

    let client = Client::new();
    match client.post(server_url).multipart(form).send() {
        Ok(response) => match response.text() {
            Ok(text) => println!("Server Response: {}", text),
            Err(_) => println!("Failed to read server response."),
        },
        Err(err) => println!("Request failed: {:?}", err),
    }
}
