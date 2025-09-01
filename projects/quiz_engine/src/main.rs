use csv::ReaderBuilder;
use serde::Deserialize;
use std::fs::File;
use std::io::{self, Write};

#[derive(Debug, Deserialize)]
struct Question {
    question: String,
    answer: String,
}

fn main() {
    let file = File::open("questions.csv").expect("❌ Failed to open CSV file");
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    // Read all questions into a vector
    let questions: Vec<Question> = reader.deserialize()
        .collect::<Result<Vec<Question>, _>>()
        .expect("❌ Failed to parse CSV data");

    println!("🧠 Welcome to the Rust Quiz!");
    println!("----------------------------\n");

    // Display level options
    println!("Select difficulty level:");
    println!("1. Beginner (10 questions)");
    println!("2. Intermediate (20 questions)");
    println!("3. Hard (50 questions)");
    println!("4. All questions ({})", questions.len());
    print!("Enter your choice (1-4): ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    let num_questions = match choice {
        "1" => {
            println!("\n🎯 Beginner level selected (10 questions)\n");
            10
        }
        "2" => {
            println!("\n🎯 Intermediate level selected (20 questions)\n");
            20
        }
        "3" => {
            println!("\n🎯 Hard level selected (50 questions)\n");
            50
        }
        "4" => {
            println!("\n🎯 All questions selected ({} questions)\n", questions.len());
            questions.len()
        }
        _ => {
            println!("\n❌ Invalid choice. Defaulting to Beginner level (10 questions)\n");
            10
        }
    };

    // Limit the number of questions to the available amount
    let num_questions = num_questions.min(questions.len());
    let mut score = 0;

    // Ask the selected number of questions
    for i in 0..num_questions {
        let q = &questions[i];
        println!("❓ Question {}/{}: {}", i + 1, num_questions, q.question);
        print!("Your answer: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let user_answer = input.trim();

        if user_answer.eq_ignore_ascii_case(&q.answer) {
            println!("✅ Correct!\n");
            score += 1;
        } else {
            println!("❌ Incorrect! Correct answer: {}\n", q.answer);
        }
    }

    println!("📊 Quiz completed!");
    println!("🎯 Your score: {}/{}", score, num_questions);
    
    let percentage = (score as f32 / num_questions as f32) * 100.0;
    println!("📈 Percentage: {:.1}%", percentage);
}