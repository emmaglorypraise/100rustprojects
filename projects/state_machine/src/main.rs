use std::io::{self, Write};


#[derive(Debug, Clone)]
enum State {
    Start,
    EnterName,
    EnterEmail(String),
    Confirm { name: String, email: String },
    Complete,
}

fn main() {
    println!("🔄 State Machine: Signup Wizard");
    
    let mut state = State::Start;
    let mut attempts = 0;
    const MAX_ATTEMPTS: u8 = 3;

    loop {
        if attempts >= MAX_ATTEMPTS {
            println!("❌ Too many failed attempts. Exiting.");
            break;
        }

        match state {
            State::Start => {
                println!("Welcome! Let's begin your signup.");
                state = State::EnterName;
            }
            State::EnterName => {
                let name = input("Enter your name: ");
                if name.is_empty() {
                    println!("❌ Name cannot be empty.");
                    attempts += 1;
                } else {
                    state = State::EnterEmail(name);
                    attempts = 0; 
                }
            }
            State::EnterEmail(ref name) => {
                let email = input("Enter your email: ");
                if validate_email(&email) {
                    state = State::Confirm { 
                        name: name.clone(), 
                        email 
                    };
                    attempts = 0;
                } else {
                    println!("❌ Invalid email format.");
                    attempts += 1;
                }
            }
            State::Confirm { name, email } => {
                println!("✅ Confirm your info:");
                println!("Name: {}", name);
                println!("Email: {}", email);
                
                let confirm = input("Is this correct? (yes/no): ").to_lowercase();
                state = match confirm.as_str() {
                    "yes" | "y" => State::Complete,
                    "no" | "n" => State::EnterName,
                    _ => {
                        println!("❌ Please answer 'yes' or 'no'.");
                        State::Confirm { name, email }
                    }
                };
            }
            State::Complete => {
                println!("🎉 Signup complete! Welcome aboard!");
                break;
            }
        }
    }
}

fn validate_email(email: &str) -> bool {
    email.contains('@') && email.contains('.') && email.len() > 5
}

fn input(prompt: &str) -> String {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        
        let mut buf = String::new();
        if io::stdin().read_line(&mut buf).is_err() {
            println!("❌ Error reading input. Please try again.");
            continue;
        }
        
        let input = buf.trim().to_string();
        if !input.is_empty() {
            return input;
        }
        
        println!("❌ Input cannot be empty.");
    }
}