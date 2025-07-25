use std::f64::consts::PI;
use std::io::{self, Write};
 
trait Shape {
    fn name(&self) -> &str;
    fn area(&self) -> f64;
}
 
struct Circle {
    radius: f64,
}
impl Shape for Circle {
    fn name(&self) -> &str {
        "Circle"
    }
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}
 
struct Rectangle {
    width: f64,
    height: f64,
}
impl Shape for Rectangle {
    fn name(&self) -> &str {
        "Rectangle"
    }
    fn area(&self) -> f64 {
        self.width * self.height
    }
}
 
struct Triangle {
    base: f64,
    height: f64,
}
impl Shape for Triangle {
    fn name(&self) -> &str {
        "Triangle"
    }
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}
 
fn main() {
    println!("🔷 Shape Area Calculator");
 
    let mut shapes: Vec<Box<dyn Shape>> = Vec::new();
 
    loop {
        println!("\n1. Add Circle\n2. Add Rectangle\n3. Add Triangle\n4. Show All Areas\n5. Exit");
        match input("Choose an option: ").as_str() {
            "1" => {
                let r = input("Enter radius: ").parse::<f64>().unwrap_or(0.0);
                shapes.push(Box::new(Circle { radius: r }));
            }
            "2" => {
                let w = input("Enter width: ").parse::<f64>().unwrap_or(0.0);
                let h = input("Enter height: ").parse::<f64>().unwrap_or(0.0);
                shapes.push(Box::new(Rectangle { width: w, height: h }));
            }
            "3" => {
                let b = input("Enter base: ").parse::<f64>().unwrap_or(0.0);
                let h = input("Enter height: ").parse::<f64>().unwrap_or(0.0);
                shapes.push(Box::new(Triangle { base: b, height: h }));
            }
            "4" => {
                for (i, shape) in shapes.iter().enumerate() {
                    println!("{}. {} Area = {:.2}", i + 1, shape.name(), shape.area());
                }
            }
            "5" => {
                println!("👋 Exiting.");
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
