use std::io;
use rand::Rng;

fn main() {
    println!("🎮 Welcome to Rock-Paper-Scissors!");
    println!("Instructions: Enter 'rock', 'paper' or 'scissors'. Type 'quit' to exit.");

    let mut user_score = 0;
    let mut comp_score = 0;


    loop {
        println!("\n🪨 📄 ✂️  Make your choice:");

        let user_choice = get_user_choice();

        if user_choice == "quit" {
            println!("👋 Thanks for playing! Goodbye!");
            break;
        }

        let computer_choice = get_computer_choice();
         println!("🤖 Computer chose: {}", computer_choice);

         match determine_winner(&user_choice, &computer_choice) {
            GameResult::Win => {
                println!("🏆 You win this round!");
                user_score += 1;
            }
            GameResult::Lose => {
                println!("😔 You lose this round!");
                comp_score += 1;
            }
            GameResult::Draw => {
                println!("🤝 It's a draw!");
            }
        }
            println!("📊 Score --> You: {} | Computer: {}", user_score, comp_score);
    }
}

fn get_user_choice() -> String {
    let mut choice = String::new();
    io::stdin()
     .read_line(&mut choice)
     .expect("Failed to get input");

    let choice = choice.trim().to_lowercase();
    match choice.as_str() {
        "rock" | "paper" | "scissors" | "quit" => choice,
        _ => {
            println!("Invalid choice. Please enter 'rock', 'paper', 'scissors' or type 'quit' to exit.");
            get_user_choice()
        }
    }
}


fn get_computer_choice() -> String {
    let choices = ["rock", "paper", "scissors"];
    let index = rand::thread_rng().gen_range(0..choices.len());
    choices[index].to_string()
}

enum GameResult {
    Win,
    Lose,
    Draw,
}

fn determine_winner(user: &str, computer: &str) -> GameResult {
    match (user, computer) {
        ("rock", "scissors") => GameResult::Win,
        ("paper", "rock") => GameResult::Win,
        ("scissors", "paper") => GameResult::Win,
        (a, b) if a == b => GameResult::Draw,
        _ => GameResult::Lose,
    }
}