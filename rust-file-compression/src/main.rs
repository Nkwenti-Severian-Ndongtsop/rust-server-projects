use flate2::write::GzEncoder;
use flate2::Compression;
use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

/// Compresses a file using Gzip compression and writes it to the output file
fn compress_file(input_path: &str, output_path: &str) -> std::io::Result<()> {
    // Open the input file
    let input_file = File::open(input_path)?;
    let mut reader = BufReader::new(input_file);

    // Create the output file
    let output_file = File::create(output_path)?;
    let writer = BufWriter::new(output_file);

    // Create a GzEncoder to compress the data
    let mut encoder = GzEncoder::new(writer, Compression::best());

    // Read data from the input file and write compressed data to output
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    encoder.write_all(&buffer)?;
    encoder.finish()?;

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = args[1].as_str(); // Change to the actual input file path
    let output_path = args[2].as_str(); // Change to desired output file path

    if let Err(e) = compress_file(input_path, output_path) {
        eprintln!("Error: {}", e);
    }
}
