use rand::{thread_rng, Rng};
 
const WIDTH: usize = 80;
const HEIGHT: usize = 24;
 
const CHARSET: &[u8] = b"@%#*+=-:. "; // Dark to light
 
fn map_brightness(value: f64) -> char {
    let index = (value * (CHARSET.len() - 1) as f64).round() as usize;
    CHARSET[index] as char
}
 
fn generate_ascii_wave() {
    println!("🎨 Generating ASCII wave pattern...\n");
 
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let fx = x as f64 / WIDTH as f64 * 4.0 * std::f64::consts::PI;
            let fy = y as f64 / HEIGHT as f64 * 4.0 * std::f64::consts::PI;
            let brightness = (fx.sin() + fy.cos() + 2.0) / 4.0; // Normalize to 0–1
            print!("{}", map_brightness(brightness));
        }
        println!();
    }
}
 
fn generate_random_static() {
    println!("✨ Generating random static...\n");
    let mut rng = thread_rng();
    for _ in 0..HEIGHT {
        for _ in 0..WIDTH {
            let value: f64 = rng.r#gen();
            print!("{}", map_brightness(value));
        }
        println!();
    }
}
 
fn main() {
    println!("🎭 ASCII Art Generator");
    println!("1) Wave Pattern");
    println!("2) Random Static");
 
    print!("Choose a mode: ");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
 
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
 
    match input.trim() {
        "1" => generate_ascii_wave(),
        "2" => generate_random_static(),
        _ => println!("❌ Invalid choice"),
    }
}
