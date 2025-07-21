use std::io::{self, Write};

pub fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

pub fn get_number_input(prompt: &str) -> Result<f64, String> {
    loop {
        let input = get_input(prompt);
        match input.parse() {
            Ok(num) if num > 0.0 => return Ok(num),
            _ => println!("Please enter a valid positive number"),
        }
    }
}

pub fn get_currency_input() -> Result<crate::models::Currency, String> {
    loop {
        println!("Select currency:");
        println!("1. Naira");
        println!("2. Dollar");
        let choice = get_input("Enter choice (1-2): ");
        match choice.as_str() {
            "1" => return Ok(crate::models::Currency::Naira),
            "2" => return Ok(crate::models::Currency::Dollar),
            _ => println!("Invalid choice"),
        }
    }
}

pub fn get_confirmation(prompt: &str) -> bool {
    let response = get_input(prompt).to_lowercase();
    response == "y" || response == "yes"
}