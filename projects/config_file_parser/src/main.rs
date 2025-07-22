use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    println!("🗂️ Config File Parser (Key=Value Format)");
    
    loop {
        let file_path = prompt("Enter path to config file (or 'exit' to quit): ")?;
        
        if file_path.to_lowercase() == "exit" {
            println!("👋 Exiting config parser");
            break;
        }

        match process_config_file(&file_path) {
            Ok(config) => {
                if config.is_empty() {
                    println!("⚠️  Config file is empty or contains no valid entries");
                } else {
                    print_config(&config);
                }
            }
            Err(e) => println!("❌ Error: {}", e),
        }
    }
    
    Ok(())
}

fn process_config_file(file_path: &str) -> io::Result<HashMap<String, String>> {
    let content = fs::read_to_string(file_path)?;
    let config = parse_config(&content);
    
    // Validate file extension if needed
    if !is_valid_config_file(file_path) {
        println!("⚠️  Warning: File extension not recognized");
    }
    
    Ok(config)
}

fn parse_config(content: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();

    for (line_num, line) in content.lines().enumerate() {
        let line = line.trim();
        
        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
            continue;
        }

        match parse_config_line(line) {
            Ok((key, value)) => {
                map.insert(key, value);
            }
            Err(e) => {
                println!("⚠️  Line {}: {}", line_num + 1, e);
            }
        }
    }

    map
}

fn parse_config_line(line: &str) -> Result<(String, String), &'static str> {
    let parts: Vec<&str> = line.splitn(2, '=').collect();
    
    if parts.len() != 2 {
        return Err("Invalid format - expected 'key=value'");
    }
    
    let key = parts[0].trim().to_string();
    let value = parts[1].trim().to_string();
    
    if key.is_empty() {
        return Err("Empty key not allowed");
    }
    
    Ok((key, value))
}

fn is_valid_config_file(file_path: &str) -> bool {
    let ext = Path::new(file_path)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    matches!(ext.as_str(), "txt" | "cfg" | "conf" | "ini")
}

fn print_config(config: &HashMap<String, String>) {
    println!("\n✅ Parsed Config ({} entries):", config.len());
    println!("{:-<30}", "");
    
    let mut keys: Vec<_> = config.keys().collect();
    keys.sort();
    
    for key in keys {
        println!("{:.<20} {}", key, config[key]);
    }
}

fn prompt(msg: &str) -> io::Result<String> {
    print!("{}", msg);
    io::stdout().flush()?;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf.trim().to_string())
}