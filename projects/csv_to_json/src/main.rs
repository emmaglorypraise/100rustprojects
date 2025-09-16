use csv::ReaderBuilder;
use serde_json::to_string_pretty;
use std::env;
use std::fs::File;
 
#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Record {
    name: String,
    email: String,
    age: u32,
}
 
fn main() {
    let args: Vec<String> = env::args().collect();
 
    if args.len() != 3 {
        eprintln!("Usage: {} <input.csv> <output.json>", args[0]);
        return;
    }
 
    let input_path = &args[1];
    let output_path = &args[2];
 
    let file = File::open(input_path).expect("❌ Failed to open CSV file");
    let mut reader = ReaderBuilder::new().from_reader(file);
 
    let mut records = Vec::new();
    for result in reader.deserialize::<Record>() {
        let record = result.expect("❌ Failed to parse a row");
        records.push(record);
    }
 
    let json = to_string_pretty(&records).expect("❌ Failed to serialize JSON");
    std::fs::write(output_path, json).expect("❌ Failed to write JSON");
 
    println!("✅ Converted {} → {}", input_path, output_path);
}