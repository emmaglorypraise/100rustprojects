use chrono::{DateTime, NaiveDateTime, TimeZone};
use chrono_tz::Tz;
use std::io::{self, Write};
 
fn main() {
    println!("🌍 Time Zone Converter");
 
    // Step 1: Ask for input datetime
    print!("🕒 Enter datetime (YYYY-MM-DD HH:MM): ");
    io::stdout().flush().unwrap();
    let mut datetime_input = String::new();
    io::stdin().read_line(&mut datetime_input).unwrap();
    let datetime_input = datetime_input.trim();
 
    // Step 2: Ask for source and target timezones
    print!("🌐 Source timezone (e.g. UTC, America/New_York): ");
    io::stdout().flush().unwrap();
    let mut src_tz_input = String::new();
    io::stdin().read_line(&mut src_tz_input).unwrap();
    let src_tz_input = src_tz_input.trim();
 
    print!("🌐 Target timezone (e.g. Asia/Tokyo): ");
    io::stdout().flush().unwrap();
    let mut tgt_tz_input = String::new();
    io::stdin().read_line(&mut tgt_tz_input).unwrap();
    let tgt_tz_input = tgt_tz_input.trim();
 
    // Step 3: Parse everything
    let naive = NaiveDateTime::parse_from_str(datetime_input, "%Y-%m-%d %H:%M");
    let src_tz: Result<Tz, _> = src_tz_input.parse();
    let tgt_tz: Result<Tz, _> = tgt_tz_input.parse();
 
    match (naive, src_tz, tgt_tz) {
        (Ok(naive_dt), Ok(src_tz), Ok(tgt_tz)) => {
            let src_dt: DateTime<Tz> = src_tz.from_local_datetime(&naive_dt).single().unwrap();
            let tgt_dt: DateTime<Tz> = src_dt.with_timezone(&tgt_tz);
 
            println!("\n⏰ {} in {} → {} in {}", naive_dt, src_tz, tgt_dt.format("%Y-%m-%d %H:%M"), tgt_tz);
        }
        _ => {
            println!("❌ Invalid input. Please check your datetime or timezone formats.");
        }
    }
}