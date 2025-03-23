use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

/// Compresses a file using Gzip compression and writes it to the output file
fn compress_file(
    input_path: &String,
    output_path: &String,
    method: &String,
) -> std::io::Result<()> {
    // Open the input file
    let input_file = File::open(input_path)?;
    let mut reader = BufReader::new(input_file);

    // Create the output file
    let output_file = File::create(output_path)?;
    let writer = BufWriter::new(output_file);

    // Create a GzEncoder to compress the data
    let mut encoder = match method.as_str() {
        "best" => GzEncoder::new(writer, Compression::best()),
        "fast" => GzEncoder::new(writer, Compression::fast()),
        "default" => GzEncoder::new(writer, Compression::default()),

        _ => GzEncoder::new(writer, Compression::best()),
    };

    // Read data from the input file and write compressed data to output
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    encoder.write_all(&buffer)?;
    encoder.finish()?;

    Ok(())
}

fn main() {
    let mut input_files: Vec<String> = Vec::new();
    let mut output_files: Vec<String> = Vec::new();
    let mut comp_meth: Vec<String> = Vec::new();
    loop {
        let mut response = String::new();
        let mut i_file = String::new();
        let mut o_file = String::new();
        let mut comp_method = String::new();
        println!("Do you want to compress files (y/n)");

        std::io::stdin()
            .read_line(&mut response)
            .expect("Failed to take response");

        if response.trim() == "n".to_string() {
            break;
        } else if response.trim() == "y".to_string() {
            println!("Enter the file to be compressed");
            std::io::stdin()
                .read_line(&mut i_file)
                .expect("Failed to take file");
            input_files.push(i_file.trim().to_string());

            println!("Enter path to store compressed file");
            std::io::stdin()
                .read_line(&mut o_file)
                .expect("Failed to take output path");
            output_files.push(o_file.trim().to_string());

            println!("Enter compression method");
            std::io::stdin()
                .read_line(&mut comp_method)
                .expect("Failed to take compression method");
            comp_meth.push(comp_method.trim().to_string());
        }
    }

    let mut index = 0;

    for _ in 0..output_files.len() {
        let (input, output, method) =
            (&input_files[index], &output_files[index], &comp_meth[index]);

        let result = compress_file(input, output, method);

        match result  {
            Ok(_) => {
                println!("The file number {} was compressed successfully", index + 1)
            }
            Err(error) => { eprint!("Error: {}", error)},
        }

        index += 1
    }
}
