use std::error::Error;
use std::fs::File;
use std::io::{self, Write};
use csv::ReaderBuilder;
 
fn main() {
    println!("📄 CSV File Processor");
 
    let file_path = input("Enter path to .csv file: ");
    let column_name = input("Enter column to filter by: ");
    let keyword = input("Enter value to match: ");
 
    if let Err(e) = filter_csv(&file_path, &column_name, &keyword) {
        eprintln!("❌ Error: {}", e);
    }
}
 
fn filter_csv(path: &str, column: &str, value: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(path)?;
    let mut reader = ReaderBuilder::new().from_reader(file);
 
    let headers = reader.headers()?.clone();
    let col_index = headers.iter().position(|h| h == column)
        .ok_or("Column not found")?;
 
    println!("\n✅ Matching rows:");
    println!("{}", headers.iter().collect::<Vec<_>>().join(","));
 
    for result in reader.records() {
        let record = result?;
        if record.get(col_index).unwrap_or("") == value {
            println!("{}", record.iter().collect::<Vec<_>>().join(","));
        }
    }
 
    Ok(())
}
 
fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
