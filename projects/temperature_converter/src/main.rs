use std::io;

fn main() {
    println!("🌡️ Welcome to Your Personal Temperature Converter!");
    println!("1: Celsius to Fehrenheit");
    println!("2: Fehrenheit to Celsius");
    println!("Please select an option (1 or 2):");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");

    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("❌ invalid choice. Please enter 1 or 2.");
            return;
        }
    };

    if choice == 1 {
        celsius_to_fehrenheit();
    } else if choice == 2 {
        fehrenheit_to_celsius();
    } else {
         println!("❌ invalid choice. Please enter 1 or 2.");
    }
}

fn celsius_to_fehrenheit() {
    println!("Enter temprature in Celsius:");

    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read input");

     let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("❌ invalid input. Please enter a valid number.");
            return;
        }
    };
    let fehrenheit = (temp * 9.0/5.0) + 32.0;
    println!("{:.2}°C is {:.2}°F", temp, fehrenheit);
}

fn fehrenheit_to_celsius() {
    println!("Enter temprature in Fehrenheit:");

    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read input");

     let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("❌ invalid input. Please enter a valid number.");
            return;
        }
    };
    let celsius = (temp - 32.0) * 5.0 / 9.0;
    println!("{:.2}°C is {:.2}°F", temp, celsius);
}

fn 