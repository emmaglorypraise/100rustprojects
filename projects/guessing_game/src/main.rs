use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    println!("🎲 Guess The Number Game!");
    
    loop {

    println!("\nSelect difficulty level:");
    println!("1: Easy");
    println!("2: Medium");
    println!("3: Hard");
    println!("4: Exit");
    print!("Enter your choice (1–4): ");
    io::stdout().flush().unwrap();

    let mut level_input = String::new();
    io::stdin()
        .read_line(&mut level_input)
        .expect("Failed to read input");
    let level: u32 = match level_input.trim().parse() {
        Ok(n) if (1..=4).contains(&n) => n,
        _ => {
            println!("❌ Invalid choice. Please enter 1, 2, 3, or 4.");
            continue;
        }
    };

    if level == 4 {
        println!("👋 Goodbye!");
        break;
    }

    let (max_number, max_attempts) = match level {
        1 => (20, 10),
        2 => (50, 8),
        3 => (100, 5),
        _ => unreachable!(),
    };

    let secret_number = rand::thread_rng().gen_range(1..=max_number);

    println!("I am thinking of a number between 1 and {}. You have {} attempts to guess it.",
            max_number, max_attempts);

    let mut guessed_correctly = false;

    for attempt in 1..=max_attempts {
        print!("\nAttempt {}/{} - Please input your guess: ", attempt, max_attempts);
        io::stdout().flush().unwrap();               
        
        let mut guess_input = String::new();
        io::stdin()
          .read_line(&mut guess_input)
          .expect("❌ Failed to read input");

        let guess: u32 = match guess_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("❌ Please enter a valid number");
                continue;
            }
        };

        if guess < 1 || guess >  max_number {
            println!("❌ Your guess is out of range (1–{}).",  max_number);
            continue;
        }

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("📉 Too small! Try again."),
            Ordering::Greater => println!("📈 Too much! Try again."),
            Ordering::Equal => {
                println!("🎉 Congratulations! You guessed the number in {} attempts.", attempt);
                guessed_correctly = true;
                break;
            }
        }
        }

        if !guessed_correctly {
            println!(
                "\n❌ You've used all {} attempts. The number was {}.",
                max_attempts, secret_number
            );
        }

        print!("\nPlay again? (y/n): ");
        io::stdout().flush().unwrap();  

        let mut answer = String::new();              
        io::stdin()
            .read_line(&mut answer)                  
            .expect("Failed to read input");

        match answer.trim().to_lowercase().as_str() {
            "y" | "yes" => continue,   
            _ => {
                println!("👋 Thanks for playing!");
                break;              
            }
        }
    }
}
