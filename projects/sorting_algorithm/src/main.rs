use std::io::{self, Write};

fn main() {
    println!("🔃 Generic Sorting Demo");

    loop {
        println!("\nChoose type to sort:");
        println!("1. Integers");
        println!("2. Words");
        println!("3. Exit");

        let choice = input("Your choice: ");
        match choice.as_str() {
            "1" => {
                let raw = input("Enter comma-separated integers: ");
                match parse_integers(&raw) {
                    Ok(mut nums) => {
                        bubble_sort(&mut nums);
                        println!("✅ Sorted: {:?}", nums);
                    }
                    Err(e) => println!("❌ Error: {}", e),
                }
            }
            "2" => {
                let raw = input("Enter comma-separated words: ");
                let mut words: Vec<String> = raw
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect();
                insertion_sort(&mut words);
                println!("✅ Sorted: {:?}", words);
            }
            "3" => {
                println!("👋 Exiting.");
                break;
            }
            _ => println!("❌ Invalid choice."),
        }
    }
}

/// Parse comma-separated integers with validation
fn parse_integers(input: &str) -> Result<Vec<i32>, String> {
    let mut nums = Vec::new();
    for s in input.split(',') {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            continue;
        }
        match trimmed.parse::<i32>() {
            Ok(num) => nums.push(num),
            Err(_) => return Err(format!("'{}' is not a valid integer", trimmed)),
        }
    }
    if nums.is_empty() {
        return Err("No valid integers provided".to_string());
    }
    Ok(nums)
}

/// Generic bubble sort
fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

/// Generic insertion sort
fn insertion_sort<T: PartialOrd + Clone>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let key = arr[i].clone();
        let mut j = i;
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1].clone();
            j -= 1;
        }
        arr[j] = key;
    }
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}