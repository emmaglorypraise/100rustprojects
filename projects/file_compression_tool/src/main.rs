use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;

#[derive(Debug)]
enum CompressionError {
    Io(io::Error),
    InvalidChoice,
    FileNotFound,
    InvalidExtension,
    CompressionFailed,
    DecompressionFailed,
}

impl std::fmt::Display for CompressionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompressionError::Io(e) => write!(f, "I/O error: {}", e),
            CompressionError::InvalidChoice => write!(f, "Invalid choice"),
            CompressionError::FileNotFound => write!(f, "Source file not found"),
            CompressionError::InvalidExtension => write!(f, "Invalid file extension"),
            CompressionError::CompressionFailed => write!(f, "Compression failed"),
            CompressionError::DecompressionFailed => write!(f, "Decompression failed"),
        }
    }
}

impl From<io::Error> for CompressionError {
    fn from(error: io::Error) -> Self {
        CompressionError::Io(error)
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("❌ Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), CompressionError> {
    println!("🗜️ File Compression Tool");
    println!("1. Compress a file");
    println!("2. Decompress a file");

    let choice = input("Enter your choice: ").trim().to_string();

    match choice.as_str() {
        "1" => handle_compression()?,
        "2" => handle_decompression()?,
        _ => return Err(CompressionError::InvalidChoice),
    }

    Ok(())
}

fn handle_compression() -> Result<(), CompressionError> {
    let src = get_valid_path("Source file path: ", false)?;
    let dest = get_valid_path("Output .gz file path: ", true)?;
    
    compress_file(&src, &dest)?;
    println!("✅ File compressed to '{}'", dest.display());
    Ok(())
}

fn handle_decompression() -> Result<(), CompressionError> {
    let src = get_valid_path("Source .gz file path: ", false)?;
    let dest = get_valid_path("Output decompressed file path: ", true)?;
    
    decompress_file(&src, &dest)?;
    println!("✅ File decompressed to '{}'", dest.display());
    Ok(())
}

fn compress_file(input_path: &Path, output_path: &Path) -> Result<(), CompressionError> {
    let input = File::open(input_path).map_err(|_| CompressionError::FileNotFound)?;
    let reader = BufReader::new(input);

    let output = File::create(output_path)?;
    let writer = BufWriter::new(output);

    let mut encoder = GzEncoder::new(writer, Compression::default());
    let mut buffer = Vec::new();

    BufReader::new(reader).read_to_end(&mut buffer)?;
    encoder.write_all(&buffer)?;
    encoder.finish()?;

    Ok(())
}

fn decompress_file(input_path: &Path, output_path: &Path) -> Result<(), CompressionError> {
    if input_path.extension().and_then(|s| s.to_str()) != Some("gz") {
        return Err(CompressionError::InvalidExtension);
    }

    let input = File::open(input_path).map_err(|_| CompressionError::FileNotFound)?;
    let mut decoder = GzDecoder::new(BufReader::new(input));

    let output = File::create(output_path)?;
    let mut writer = BufWriter::new(output);

    let mut buffer = Vec::new();
    decoder.read_to_end(&mut buffer)?;
    writer.write_all(&buffer)?;

    Ok(())
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn get_valid_path(prompt: &str, is_output: bool) -> Result<PathBuf, CompressionError> {
    let path_str = input(prompt);
    let path = PathBuf::from(&path_str);

    if !is_output && !path.exists() {
        return Err(CompressionError::FileNotFound);
    }

    Ok(path)
}