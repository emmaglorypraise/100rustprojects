use std::io::{self, Write};
 
trait Greeter {
    fn greet(&self, name: &str) -> String;
}
 
struct Friendly;
impl Greeter for Friendly {
    fn greet(&self, name: &str) -> String {
        format!("Hey there, {}! Great to see you!", name)
    }
}
 
struct Formal;
impl Greeter for Formal {
    fn greet(&self, name: &str) -> String {
        format!("Good day to you, {}.", name)
    }
}
 
struct Sarcastic;
impl Greeter for Sarcastic {
    fn greet(&self, name: &str) -> String {
        format!("Oh wow, {} showed up. Amazing.", name)
    }
}
 
fn main() {
    println!("🧠 Trait Object Demo - Pick a Personality!");
 
    let mut greeter: Box<dyn Greeter> = Box::new(Friendly); // default
 
    loop {
        println!("\nChoose a mode:");
        println!("1. Friendly\n2. Formal\n3. Sarcastic\n4. Greet\n5. Exit");
 
        match input("Option: ").as_str() {
            "1" => {
                greeter = Box::new(Friendly);
                println!("✅ Switched to Friendly mode.");
            }
            "2" => {
                greeter = Box::new(Formal);
                println!("✅ Switched to Formal mode.");
            }
            "3" => {
                greeter = Box::new(Sarcastic);
                println!("✅ Switched to Sarcastic mode.");
            }
            "4" => {
                let name = input("Enter your name: ");
                println!("{}", greeter.greet(&name));
            }
            "5" => {
                println!("👋 Bye!");
                break;
            }
            _ => println!("❌ Invalid choice."),
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