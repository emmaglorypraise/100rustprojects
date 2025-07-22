use std::error::Error;
use std::fmt;
use std::io::{self, Write};
 
fn main() {
    println!("🧮 Custom Error Handling: Square Root Calculator");
 
    let input = prompt("Enter a number: ");
    match input.trim().parse::<f64>() {
        Ok(num) => match calculate_sqrt(num) {
            Ok(result) => println!("✅ Square root: {}", result),
            Err(e) => eprintln!("❌ {}", e),
        },
        Err(_) => eprintln!("❌ Invalid number format."),
    }
}
 
/// Custom error type
#[derive(Debug)]
enum MathError {
    NegativeInput,
}
 
impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MathError::NegativeInput => write!(f, "Cannot calculate the square root of a negative number."),
        }
    }
}
 
impl Error for MathError {}
 
/// Calculate square root with custom error
fn calculate_sqrt(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        Err(MathError::NegativeInput)
    } else {
        Ok(x.sqrt())
    }
}
 
/// Get input from user
fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}