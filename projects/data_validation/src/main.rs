use regex::Regex;
use std::io::{self, Write};
use colored::Colorize;

fn main() {
    println!("{}", "✅ Data Validation Tool".bright_green());

    let email_regex = Regex::new(r"^[\w\.-]+@[\w\.-]+\.\w+$").unwrap();
    let phone_regex = Regex::new(r"^\+?[0-9]{10,15}$").unwrap();

    loop {
        println!("\nChoose what to validate:");
        println!("1. Email");
        println!("2. Phone Number");
        println!("3. Password Strength");
        println!("4. Exit");

        match input("Your choice: ").as_str() {
            "1" => validate_email(&email_regex),
            "2" => validate_phone(&phone_regex),
            "3" => validate_password(),
            "4" => {
                println!("{}", "👋 Exiting.".bright_blue());
                break;
            }
            _ => println!("{}", "❌ Invalid choice.".bright_red()),
        }
    }
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn validate_email(email_regex: &Regex) {
    let email = input("Enter email: ");
    if email_regex.is_match(&email) {
        println!("{}", "✅ Valid email.".bright_green());
    } else {
        println!("{}", "❌ Invalid email.".bright_red());
    }
}

fn validate_phone(phone_regex: &Regex) {
    let phone = input("Enter phone number (e.g., +1234567890): ");
    if phone_regex.is_match(&phone) {
        println!("{}", "✅ Valid phone number.".bright_green());
    } else {
        println!("{}", "❌ Invalid phone number.".bright_red());
    }
}

fn validate_password() {
    let pwd = input("Enter password: ");
    match check_password_strength(&pwd) {
        Ok(true) => println!("{}", "✅ Strong password.".bright_green()),
        Ok(false) => println!(
            "{}",
            "❌ Weak password (must be 8+ chars, contain upper, lower, digit, and symbol).".bright_red()
        ),
        Err(e) => println!("{}: {}", "❌ Error validating password".bright_red(), e),
    }
}

fn check_password_strength(pwd: &str) -> Result<bool, String> {
    if pwd.len() < 8 {
        return Ok(false);
    }

    let has_lowercase = pwd.chars().any(|c| c.is_ascii_lowercase());
    let has_uppercase = pwd.chars().any(|c| c.is_ascii_uppercase());
    let has_digit = pwd.chars().any(|c| c.is_ascii_digit());
    let has_special = pwd.chars().any(|c| !c.is_ascii_alphanumeric());

    Ok(has_lowercase && has_uppercase && has_digit && has_special)
}