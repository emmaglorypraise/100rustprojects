use std::fs::{File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
 
fn main() {
    println!("📝 Rust Mini Text Editor");
    let file_path = prompt("Enter file to open or create: ");
    let mut lines = load_file(&file_path);
 
    loop {
        println!("\nCurrent file: {}", file_path);
        display_lines(&lines);
 
        println!("\nCommands:");
        println!("1. Add line");
        println!("2. Edit line");
        println!("3. Delete line");
        println!("4. Save");
        println!("5. Exit");
 
        let choice = prompt("Select an option: ");
 
        match choice.trim() {
            "1" => {
                let new_line = prompt("Enter new line: ");
                lines.push(new_line);
            }
            "2" => {
                let idx = prompt("Line number to edit: ").parse::<usize>().unwrap_or(0);
                if idx == 0 || idx > lines.len() {
                    println!("❌ Invalid line number.");
                } else {
                    let updated = prompt("Enter updated text: ");
                    lines[idx - 1] = updated;
                }
            }
            "3" => {
                let idx = prompt("Line number to delete: ").parse::<usize>().unwrap_or(0);
                if idx == 0 || idx > lines.len() {
                    println!("❌ Invalid line number.");
                } else {
                    lines.remove(idx - 1);
                }
            }
            "4" => {
                if let Err(e) = save_file(&file_path, &lines) {
                    println!("❌ Save failed: {}", e);
                } else {
                    println!("✅ File saved.");
                }
            }
            "5" => {
                println!("👋 Exiting editor.");
                break;
            }
            _ => println!("❌ Unknown command."),
        }
    }
}
 
fn load_file(path: &str) -> Vec<String> {
    if Path::new(path).exists() {
        let file = File::open(path).unwrap();
        BufReader::new(file)
            .lines()
            .filter_map(Result::ok)
            .collect()
    } else {
        vec![]
    }
}
 
fn save_file(path: &str, lines: &[String]) -> io::Result<()> {
    let mut file = File::create(path)?;
    for line in lines {
        writeln!(file, "{}", line)?;
    }
    Ok(())
}
 
fn display_lines(lines: &[String]) {
    println!("\n---- File Content ----");
    for (i, line) in lines.iter().enumerate() {
        println!("{:>3}: {}", i + 1, line);
    }
}
 
fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}