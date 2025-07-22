mod bot;

use std::io;
use std::io::Write;
use crate::bot::ChatBot;

fn main() -> io::Result<()> {
    println!("💬 ChatBot CLI - Type 'exit' to quit or 'help' for commands");
    
    let mut bot = ChatBot::new();
    bot.history.load()?;

    loop {
        let input = prompt("You: ")?;
        
        match input.to_lowercase().as_str() {
            "exit" => {
                println!("👋 Goodbye!");
                bot.history.save()?;
                break;
            }
            "help" => print_help(),
            "history" => bot.history.print(),
            "clear" => {
                bot.history.clear()?;
                println!("🗑️ Chat history cleared!");
            }
            _ => {
                let response = bot.process_message(&input);
                println!("Bot: {response}");
            }
        }
    }

    Ok(())
}

fn print_help() {
    println!("\n📖 Available Commands:");
    println!("- 'help': Show this help message");
    println!("- 'history': Show chat history");
    println!("- 'clear': Clear chat history");
    println!("- 'exit': Quit the chat");
    println!("\n💡 Try asking about: hello, how are you, rust, time, date");
}

fn prompt(msg: &str) -> io::Result<String> {
    print!("{msg}");
    io::stdout().flush()?;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf.trim().to_string())
}