use clap::{Arg, Command};
use reqwest::blocking::{
    multipart::{Form, Part},
    Client,
};
use std::{fs::File, io::Read};

fn main() {
    let matches = Command::new("File Uploader")
        .arg(
            Arg::new("compress")
                .short('c')
                .num_args(1..)
                .help("Files to compress & upload"),
        )
        .arg(
            Arg::new("method")
                .short('m')
                .default_value("default")
                .help("Compression method: best, fast, or default"),
        )
        .get_matches();

    let files: Vec<&str> = matches
        .get_many::<String>("compress")
        .unwrap_or_default()
        .map(String::as_str)
        .collect();

    let method = matches.get_one::<String>("method").unwrap();

    let server_url = "http://127.0.0.1:8000/upload";
    let client = Client::new();

    for file_path in files {
        match File::open(file_path) {
            Ok(mut file) => {
                let mut buffer = Vec::new();
                if file.read_to_end(&mut buffer).is_ok() {
                    let filename = file_path.split('/').next_back().unwrap_or("unknown_file");

                    let form = Form::new()
                        .part("file", Part::bytes(buffer).file_name(filename.to_string()))
                        .text("method", method.to_string());

                    match client.post(server_url).multipart(form).send() {
                        Ok(response) => match response.text() {
                            Ok(text) => {
                                println!("Uploaded {} - Server Response: {}", filename, text)
                            }
                            Err(_) => println!("Failed to read server response."),
                        },
                        Err(err) => println!("Request failed for {}: {:?}", filename, err),
                    }
                }
            }
            Err(_) => println!("Could not open file: {}", file_path),
        }
    }
}
