use std::fs::OpenOptions;
use std::io::{self, Write};
use chrono::Local;
 
fn main() {
    println!("📝 Logger Utility (writes to log.txt)");
 
    loop {
        println!("\nLog Levels: 1. INFO | 2. WARN | 3. ERROR | 4. Exit");
        let choice = input("Select a level: ");
 
        match choice.as_str() {
            "1" => log_message("INFO"),
            "2" => log_message("WARN"),
            "3" => log_message("ERROR"),
            "4" => {
                println!("👋 Exiting logger.");
                break;
            }
            _ => println!("❌ Invalid choice."),
        }
    }
}
 
fn log_message(level: &str) {
    let message = input(&format!("Enter {} message: ", level));
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let entry = format!("[{}] {}: {}\n", timestamp, level, message);
 
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("log.txt")
        .expect("❌ Unable to open log file");
 
    file.write_all(entry.as_bytes()).expect("❌ Write failed");
    println!("✅ Logged successfully.");
}
 
fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}