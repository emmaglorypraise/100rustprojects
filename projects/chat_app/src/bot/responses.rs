use chrono::Local;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
  pub static ref RESPONSES: HashMap<String, String> = {
     let mut map = HashMap::new();
        map.insert(
            "what is your name".to_string(), 
            "I'm Rusty, your friendly Rust-powered chatbot! 🤖".to_string()
        );
        map.insert(
            "who are you".to_string(),
            "I'm your personal Rust assistant here to help with programming and motivation!".to_string()
        );
        map.insert(
            "motivation".to_string(), 
            "Remember: Every expert was once a beginner. Keep coding and you'll get there! 💪".to_string()
        );
        map.insert(
            "inspire me".to_string(), 
            "The only way to do great work is to love what you do. - Steve Jobs ✨".to_string()
        );
        map.insert(
            "where are you from".to_string(), 
            "I was born in the digital cloud, powered by Rust's amazing ecosystem! ☁️".to_string()
        );
        map.insert(
            "what do you think about rust".to_string(), 
            "Rust is revolutionary! It gives you:\n- Memory safety without GC\n- Fearless concurrency\n- Blazing fast performance\n- An amazing community 🦀".to_string()
        );
        map.insert(
            "how to learn rust".to_string(), 
            "Here's the best path:\n1. Read 'The Book' (official Rust documentation)\n2. Do Rustlings exercises\n3. Build small projects\n4. Contribute to open-source\n5. Never stop learning! 📚".to_string()
        );
        map.insert(
            "best rust features".to_string(), 
            "My favorite Rust features are:\n- Ownership system\n- Pattern matching\n- Zero-cost abstractions\n- Cargo package manager\n- Helpful compiler messages".to_string()
        );
        map.insert(
            "rust vs python".to_string(), 
            "Both are great! Rust gives you performance and safety, while Python offers simplicity. Use Rust for:\n- System programming\n- Performance-critical code\n- Concurrency\nUse Python for:\n- Rapid prototyping\n- Data science\n- Scripting".to_string()
        );
        map.insert(
            "thank you".to_string(), 
            "You're very welcome! Happy to help. Keep rocking with Rust! 🦀".to_string()
        );
        map.insert(
            "bye".to_string(), 
            "Goodbye! Remember: The only bad code is the code you don't write! 🚀".to_string()
        );
        map.insert(
            "time".to_string(), 
            format!("Current time is {}", Local::now().format("%H:%M"))
        );
        map.insert(
            "date".to_string(), 
            format!("Today is {}", Local::now().format("%Y-%m-%d"))
        );
        map.insert(
            "who created you".to_string(),
            "I was created by a Rust enthusiast to demonstrate Rust's capabilities for building CLI applications!".to_string()
        );
        map.insert(
            "joke".to_string(),
            "Why did the Rustacean break up with Python?\nBecause it wanted to borrow, not reference! 😄".to_string()
        );
        map.insert(
            "tip".to_string(),
            "Pro tip: Always run `cargo clippy` - it's like having a senior Rust developer looking over your shoulder!".to_string()
        );
        map.insert(
          "hello".to_string(), 
          "Hello there 🤖".to_string()
        );
        map.insert(
          "what's up|hi|hey".to_string(), 
          "What's happening?".to_string()
        );
        map.insert(
            "about rust|tell me about rust".to_string(),
            "Rust is a systems programming language...".to_string()
        );


        map

  };

}
       