use std::io::{self, Write};

pub fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

pub fn get_number_input(prompt: &str) -> Option<usize> {
    loop {
        let input = get_input(prompt);
        if input.is_empty() {
            return None;
        }
        match input.parse() {
            Ok(num) => return Some(num),
            Err(_) => println!("Please enter a valid number."),
        }
    }
}