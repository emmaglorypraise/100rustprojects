use std::env;
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::path::Path;

fn get_terminal_width() -> usize {
    termsize::get()
        .map(|size| size.cols as usize)
        .unwrap_or(80) // Default width if cannot determine
}

fn print_separator() {
    let width = get_terminal_width();
    println!("{}", "─".repeat(width));
}

fn main() {
    println!("\n🖥️   Rust Mini Shell (type 'exit' to quit)");
    print_separator();

    loop {
        // Get current directory and format prompt
        let current_dir = env::current_dir()
            .unwrap_or_else(|_| Path::new("?").to_path_buf())
            .display()
            .to_string();
        
        let prompt = if let Ok(home) = env::var("HOME") {
            current_dir.replace(&home, "~")
        } else {
            current_dir
        };

        print!("rust-shell:{}> ", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("❌ Failed to read input.");
            print_separator();
            continue;
        }

        let input = input.trim();
        if input.is_empty() {
            print_separator();
            continue;
        }

        if input == "exit" {
            println!("👋 Exiting shell.");
            break;
        }

        if input.starts_with("cd ") {
            let path = input.strip_prefix("cd ").unwrap().trim();
            if path.is_empty() {
                if let Ok(home) = env::var("HOME") {
                    let _ = env::set_current_dir(home);
                }
                print_separator();
                continue;
            }
            if let Err(e) = env::set_current_dir(path) {
                println!("❌ cd failed: {}", e);
            }
            print_separator();
            continue;
        }

        if input == "pwd" {
            match env::current_dir() {
                Ok(path) => println!("{}", path.display()),
                Err(e) => println!("❌ pwd failed: {}", e),
            }
            print_separator();
            continue;
        }

        run_command(input);
        print_separator();
    }
}

fn run_command(command_line: &str) {
    let parts: Vec<&str> = command_line.split_whitespace().collect();
    if parts.is_empty() {
        return;
    }

    let (cmd, args) = parts.split_first().unwrap();

    match Command::new(cmd)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
    {
        Ok(mut child) => {
            if let Err(e) = child.wait() {
                println!("❌ Command failed: {}", e);
            }
        }
        Err(e) => {
            println!("❌ Command failed: {}", e);
        }
    }
}