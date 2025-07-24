use std::io::{self, Write};
 
trait Plugin {
    fn name(&self) -> &str;
    fn run(&self, input: &str) -> String;
}
 
struct UppercasePlugin;
impl Plugin for UppercasePlugin {
    fn name(&self) -> &str {
        "Uppercase"
    }
    fn run(&self, input: &str) -> String {
        input.to_uppercase()
    }
}
 
struct ReversePlugin;
impl Plugin for ReversePlugin {
    fn name(&self) -> &str {
        "Reverse"
    }
    fn run(&self, input: &str) -> String {
        input.chars().rev().collect()
    }
}
 
struct DuplicatePlugin;
impl Plugin for DuplicatePlugin {
    fn name(&self) -> &str {
        "Duplicate"
    }
    fn run(&self, input: &str) -> String {
        format!("{}{}", input, input)
    }
}
 
fn main() {
    println!("🔌 Plugin System Demo");
 
    let plugins: Vec<Box<dyn Plugin>> = vec![
        Box::new(UppercasePlugin),
        Box::new(ReversePlugin),
        Box::new(DuplicatePlugin),
    ];
 
    loop {
        println!("\nAvailable Plugins:");
        for (i, plugin) in plugins.iter().enumerate() {
            println!("{}. {}", i + 1, plugin.name());
        }
        println!("{}. Exit", plugins.len() + 1);
 
        let choice = input("Choose a plugin: ");
        let index = match choice.trim().parse::<usize>() {
            Ok(num) if num >= 1 && num <= plugins.len() => num - 1,
            Ok(num) if num == plugins.len() + 1 => {
                println!("👋 Exiting.");
                break;
            }
            _ => {
                println!("❌ Invalid choice.");
                continue;
            }
        };
 
        let data = input("Enter input text: ");
        let result = plugins[index].run(&data);
        println!("🧠 Plugin Output: {}", result);
    }
}
 
fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
