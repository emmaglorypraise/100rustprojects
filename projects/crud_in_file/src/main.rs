use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

const FILE_PATH: &str = "notes.txt";

fn main() -> io::Result<()> {
    loop {
        println!("\n📝 Text File CRUD Menu:");
        println!("1. Create/Overwrite");
        println!("2. Read");
        println!("3. Update line");
        println!("4. Delete line");
        println!("5. Exit");

        let choice = input("Choose an option: ")?;

        match choice.as_str() {
            "1" => {
                let content = input("Enter new content: ")?;
                fs::write(FILE_PATH, content)?;
                println!("✅ File saved.");
            }
            "2" => {
                if !Path::new(FILE_PATH).exists() {
                    println!("📭 File not found.");
                    continue;
                }
                
                println!("📄 File Content:");
                for (i, line) in BufReader::new(File::open(FILE_PATH)?).lines().enumerate() {
                    println!("{}: {}", i + 1, line?);
                }
            }
            "3" => {
                let lines = read_lines()?;
                let line_no = input("Line to update: ")?.parse().unwrap_or(0);
                
                if line_no == 0 || line_no > lines.len() {
                    println!("❌ Invalid line number.");
                    continue;
                }

                let new_text = input("New content: ")?;
                let mut updated = lines;
                updated[line_no - 1] = new_text;
                write_lines(&updated)?;
                println!("✅ Line updated.");
            }
            "4" => {
                let mut lines = read_lines()?;
                let line_no = input("Line to delete: ")?.parse().unwrap_or(0);
                
                if line_no == 0 || line_no > lines.len() {
                    println!("❌ Invalid line number.");
                    continue;
                }

                lines.remove(line_no - 1);
                write_lines(&lines)?;
                println!("✅ Line deleted.");
            }
            "5" => break,
            _ => println!("❌ Invalid choice."),
        }
    }
    Ok(())
}

fn read_lines() -> io::Result<Vec<String>> {
    if !Path::new(FILE_PATH).exists() {
        return Ok(Vec::new());
    }
    BufReader::new(File::open(FILE_PATH)?).lines().collect()
}

fn write_lines(lines: &[String]) -> io::Result<()> {
    let mut content = lines.join("\n");
    content.push('\n'); // Ensure trailing newline
    fs::write(FILE_PATH, content)
}

fn input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf.trim().to_string())
}