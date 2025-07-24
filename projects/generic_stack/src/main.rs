use std::fmt::{Debug, Display};
use std::io::{self, Write};

#[derive(Debug, Default)]
pub struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    /// Creates a new empty stack
    pub fn new() -> Self {
        Self::default()
    }

    /// Pushes an item onto the stack
    pub fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    /// Pops an item from the stack
    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    /// Returns a reference to the top item without removing it
    pub fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    /// Returns a mutable reference to the top item
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.elements.last_mut()
    }

    /// Checks if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Returns the number of items in the stack
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Clears all items from the stack
    pub fn clear(&mut self) {
        self.elements.clear();
    }

    /// Returns an iterator over the stack's elements (top to bottom)
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.elements.iter().rev()
    }
}

impl<T: Debug> Stack<T> {
    /// Prints the stack contents
    pub fn print(&self) {
        if self.is_empty() {
            println!("Stack is empty");
            return;
        }

        println!("Stack contents (top to bottom):");
        for (i, item) in self.iter().enumerate() {
            println!("{}. {:?}", i + 1, item);
        }
    }
}

fn main() {
    let mut stack = Stack::new();

    loop {
        println!("\nStack Operations:");
        println!("1. Push");
        println!("2. Pop");
        println!("3. Peek");
        println!("4. Peek and modify");
        println!("5. Size");
        println!("6. Print Stack");
        println!("7. Clear Stack");
        println!("8. Exit");

        let choice = get_input("Enter your choice: ");

        match choice.as_str() {
            "1" => {
                let val = get_input("Enter value to push: ");
                stack.push(val);
                println!("✅ Item pushed successfully");
            }
            "2" => match stack.pop() {
                Some(val) => println!("✅ Popped: {}", val),
                None => println!("❌ Stack is empty"),
            },
            "3" => match stack.peek() {
                Some(val) => println!("✅ Top item: {}", val),
                None => println!("❌ Stack is empty"),
            },
            "4" => match stack.peek_mut() {
                Some(val) => {
                    println!("Current top item: {}", val);
                    let new_val = get_input("Enter new value: ");
                    *val = new_val;
                    println!("✅ Top item modified");
                }
                None => println!("❌ Stack is empty"),
            },
            "5" => println!("📏 Stack size: {}", stack.len()),
            "6" => stack.print(),
            "7" => {
                stack.clear();
                println!("🧹 Stack cleared");
            }
            "8" => {
                println!("👋 Exiting program");
                break;
            }
            _ => println!("❌ Invalid option, please try again"),
        }
    }
}

/// Helper function to get user input with proper error handling
fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}