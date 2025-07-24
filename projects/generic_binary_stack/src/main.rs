use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    println!("🔍 Generic Binary Search CLI");
    
    let numbers = vec![1, 3, 5, 7, 9, 11, 13];
    let words = vec!["apple", "banana", "cherry", "date", "fig", "grape"];

    loop {
        println!("\nMain Menu:");
        println!("1. Search in Numbers");
        println!("2. Search in Words");
        println!("3. Exit");
        print!("Please choose an option (1-3): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => number_search(&numbers),
            "2" => word_search(&words),
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option. Please enter 1, 2, or 3."),
        }
    }
}

fn number_search(numbers: &[i32]) {
    loop {
        print!("Enter a number to search (or 'back' to return to menu): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("back") {
            break;
        }

        match input.parse::<i32>() {
            Ok(num) => {
                match binary_search(numbers, &num) {
                    Some(idx) => println!("✅ Found {} at index {}", num, idx),
                    None => println!("❌ {} not found in the list", num),
                }
            }
            Err(_) => println!("❌ Please enter a valid number"),
        }
    }
}

fn word_search(words: &[&str]) {
    let words_as_strings: Vec<String> = words.iter().map(|&s| s.to_string()).collect();
    
    loop {
        print!("Enter a word to search (or 'back' to return to menu): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("back") {
            break;
        }

        match binary_search(&words_as_strings, &input.to_string()) {
            Some(idx) => println!("✅ Found '{}' at index {}", input, idx),
            None => println!("❌ '{}' not found in the list", input),
        }
    }
}

fn binary_search<T: PartialOrd>(list: &[T], target: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = list.len();

    while low < high {
        let mid = low + (high - low) / 2;
        match list[mid].partial_cmp(target) {
            Some(Ordering::Equal) => return Some(mid),
            Some(Ordering::Less) => low = mid + 1,
            Some(Ordering::Greater) => high = mid,
            None => return None,
        }
    }
    None
}