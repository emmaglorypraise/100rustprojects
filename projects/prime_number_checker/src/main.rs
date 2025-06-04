use std::io;

fn main() {
    println!("🔢 Prime Number Checker");
    println!("Enter a positive integar to check if it's prime:");

    let number = match get_input_as_u32() {
        Some(value) => value,
        None => {
            println!("❌ Invalid input. Please enter a positive integar.");
            return;
        }
    };

    if number <= 1 {
        println!("❌ Number must be greater than 1.");
        return;
    }

    if is_prime(number) {
        println!("✅ {} is a prime number.", number);
    } else {
        println!("❌ {} is not a prime number.", number);
    }

    let primes = prime_numbers(number);
    println!("🔍 All prime numbers up to {}: {:?}", number, primes);

}

fn get_input_as_u32() -> Option<u32> {
    let mut input = String::new();
    io::stdin()
     .read_line(&mut input)
     .expect("Failed to get input.");

    match input.trim().parse::<u32>() {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}


fn is_prime(n: u32) -> bool {
    if n == 1 {
        return false;
    }

    if n == 2 {
        return true;
    }

    if n % 2 == 0 {
        return false;
    }

    let limit = (n as f64).sqrt() as u32 + 1;
    for i in 3..limit {
        if n % i == 0 {
            return false;
        }
    } 

    true
}

fn prime_numbers(n: u32) -> Vec<u32> {
    let mut primes_num = Vec::new();
    for num in 2..=n {
        if is_prime(num) {
            primes_num.push(num);
        }
    }

    primes_num
}