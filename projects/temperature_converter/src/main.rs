use std::io;

fn main() {
    println!("\n 🌡️ Welcome to Your Personal Temperature Converter!");

    loop {
        println!("\nSelect conversion:");
        println!("1: Celsius to Fehrenheit");
        println!("2: Fehrenheit to Celsius");
        println!("3: Kelvin to Fehrenheit");
        println!("4: Kelvin to Celsius");
        println!("5: Celsius to Kelvin");
        println!("6: Fehrenheit to Kelvin");
        println!("Please select an option (1-6):");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("❌ invalid choice. Please enter a number between 1 and 6.");
                return;
            }
        };

        if choice == 1 {
            celsius_to_fehrenheit();
        } else if choice == 2 {
            fehrenheit_to_celsius();
        }  else if choice == 3 {
            kelvin_to_fehrenheit();
        }  else if choice == 4 {
            kelvin_to_celsius()
        }  else if choice == 5 {
            celsius_to_kelvin();
        }  else if choice == 6 {
            fehrenheit_to_kelvin();
        } else {
            println!("❌ invalid choice. Please enter a number between 1 and 6.");
            break;
        }
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
    println!("-----------------------------------------");
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
    println!("{:.2}°F is {:.2}°C", temp, celsius);
    println!("-----------------------------------------");
}

fn kelvin_to_fehrenheit() {
    println!("Enter temprature in Kelvin:");

    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read input");

     let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("❌ invalid input. Please enter a valid number.");
            return;
        }
    };
    let fehrenheit = (temp - 273.15) * (9.0/5.0) + 32.0;
    println!("{:.2}°K is {:.2}°F", temp, fehrenheit);
    println!("-----------------------------------------");
}

fn kelvin_to_celsius() {
    println!("Enter temprature in Kelvin:");

    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read input");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("❌ invalid input. Please enter a valid number.");
            return;
        }
    };
    let celsius = temp - 273.15;
    println!("{:.2}°K is {:.2}°C", temp, celsius);
    println!("-----------------------------------------");
}

fn celsius_to_kelvin() {
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
    let kelvin = temp + 273.15;
    println!("{:.2}°C is {:.2}°K", temp, kelvin);
    println!("-----------------------------------------");
}

fn fehrenheit_to_kelvin() {
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
    let kelvin = (temp - 32.0) * (5.0 / 9.0) + 273.15;
    println!("{:.2}°F is {:.2}°K", temp, kelvin);
    println!("-----------------------------------------");
}

