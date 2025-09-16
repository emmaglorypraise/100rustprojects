use chrono::{NaiveDateTime, TimeZone};
use chrono_tz::Tz;
use std::io::{self, Write};
use std::collections::HashMap;

fn main() {
    println!("🌍 Time Zone Converter");
    println!("======================");
    println!("Supports: IANA names (America/New_York) AND abbreviations (EST, PST, CST, etc.)");
    println!("Type 'quit' to exit, 'now' for current time\n");

    loop {
        let datetime_input = get_input("🕒 Enter datetime (e.g., 2025-04-11 14:00 or 'now'): ");
        
        if datetime_input.to_lowercase() == "quit" || datetime_input.to_lowercase() == "exit" {
            println!("👋 Goodbye!");
            break;
        }

        let src_tz_input = get_input("🌐 Source timezone (e.g., EST, PST, UTC, America/New_York): ");
        let tgt_tz_input = get_input("🌐 Target timezone (e.g., PST, Asia/Tokyo, Europe/London): ");

        match convert_timezone(&datetime_input, &src_tz_input, &tgt_tz_input) {
            Ok(result) => println!("\n✅ {}\n", result),
            Err(e) => println!("\n❌ {}\n", e),
        }
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn convert_timezone(datetime_str: &str, src_tz_str: &str, tgt_tz_str: &str) -> Result<String, String> {
    // Parse timezones with abbreviation support
    let src_tz = parse_timezone(src_tz_str)
        .map_err(|e| format!("Source timezone: {}", e))?;
    
    let tgt_tz = parse_timezone(tgt_tz_str)
        .map_err(|e| format!("Target timezone: {}", e))?;

    // Handle "now" keyword or parse datetime
    let naive_dt = if datetime_str.to_lowercase() == "now" {
        let now_utc = chrono::Utc::now();
        let now_in_src = now_utc.with_timezone(&src_tz);
        now_in_src.naive_local()
    } else {
        parse_datetime(datetime_str)
            .ok_or_else(|| format!("Invalid datetime format: '{}'", datetime_str))?
    };

    // Convert between timezones
    let src_dt = src_tz.from_local_datetime(&naive_dt);
    
    match src_dt {
        chrono::LocalResult::Single(dt) => {
            let tgt_dt = dt.with_timezone(&tgt_tz);
            Ok(format!("{} in {} → {} in {}", 
                dt.format("%Y-%m-%d %H:%M:%S"), 
                get_timezone_display_name(src_tz_str, src_tz),
                tgt_dt.format("%Y-%m-%d %H:%M:%S"), 
                get_timezone_display_name(tgt_tz_str, tgt_tz)
            ))
        },
        chrono::LocalResult::Ambiguous(dt1, dt2) => {
            Err(format!(
                "Ambiguous time due to DST transition. Could be either:\n- {} ({})\n- {} ({})",
                dt1.with_timezone(&tgt_tz).format("%Y-%m-%d %H:%M:%S"),
                dt1.offset(),
                dt2.with_timezone(&tgt_tz).format("%Y-%m-%d %H:%M:%S"),
                dt2.offset()
            ))
        },
        chrono::LocalResult::None => {
            Err(format!(
                "Non-existent time in {} (likely due to DST spring forward)",
                get_timezone_display_name(src_tz_str, src_tz)
            ))
        },
    }
}

fn parse_timezone(tz_str: &str) -> Result<Tz, String> {
    let tz_lower = tz_str.to_uppercase();
    
    let abbreviation_map: HashMap<&str, &str> = HashMap::from([
        // US Timezones
        ("EST", "America/New_York"),
        ("EDT", "America/New_York"),
        ("CST", "America/Chicago"),
        ("CDT", "America/Chicago"),
        ("MST", "America/Denver"),
        ("MDT", "America/Denver"),
        ("PST", "America/Los_Angeles"),
        ("PDT", "America/Los_Angeles"),
        ("AKST", "America/Anchorage"),
        ("AKDT", "America/Anchorage"),
        ("HST", "Pacific/Honolulu"),
        
        // European Timezones
        ("GMT", "Europe/London"),
        ("BST", "Europe/London"),
        ("CET", "Europe/Paris"),
        ("CEST", "Europe/Paris"),
        ("EET", "Europe/Athens"),
        ("EEST", "Europe/Athens"),
        ("WET", "Europe/Lisbon"),
        ("WEST", "Europe/Lisbon"),
        
        // African Timezones
        ("WAT", "Africa/Lagos"),      // West Africa Time
        ("WAST", "Africa/Windhoek"),  // West Africa Summer Time
        ("CAT", "Africa/Harare"),     // Central Africa Time
        ("EAT", "Africa/Nairobi"),    // East Africa Time
        ("SAST", "Africa/Johannesburg"), // South Africa Standard Time
        ("GMT+1", "Africa/Lagos"),
        ("GMT+2", "Africa/Cairo"),
        ("GMT+3", "Africa/Nairobi"),
        
        // Middle East
        ("AST", "Asia/Riyadh"),       // Arabia Standard Time
        ("GST", "Asia/Dubai"),        // Gulf Standard Time
        
        // Asian Timezones
        ("IST", "Asia/Kolkata"),      // Indian Standard Time
        ("JST", "Asia/Tokyo"),
        ("CST", "Asia/Shanghai"),     // China Standard Time
        ("KST", "Asia/Seoul"),        // Korea Standard Time
        ("SGT", "Asia/Singapore"),
        
        // Australian Timezones
        ("AEST", "Australia/Sydney"),
        ("AEDT", "Australia/Sydney"),
        ("AWST", "Australia/Perth"),
        
        // UTC variations
        ("UTC", "UTC"),
        ("UT", "UTC"),
        ("Z", "UTC"),
    ]);

    // First try to parse as IANA timezone
    if let Ok(tz) = tz_str.parse::<Tz>() {
        return Ok(tz);
    }

    // Then try abbreviation mapping
    if let Some(iana_name) = abbreviation_map.get(tz_lower.as_str()) {
        return iana_name.parse::<Tz>()
            .map_err(|_| format!("Could not map abbreviation '{}'", tz_str));
    }

    Err(format!("Unknown timezone: '{}'. Use IANA names or common abbreviations.", tz_str))
}

fn get_timezone_display_name(input_str: &str, tz: Tz) -> String {
    // If user used an abbreviation, show both abbreviation and full name
    if input_str.len() <= 5 && input_str.chars().all(|c| c.is_ascii_alphabetic()) {
        format!("{} ({})", input_str.to_uppercase(), tz)
    } else {
        tz.to_string()
    }
}

fn parse_datetime(datetime_str: &str) -> Option<NaiveDateTime> {
    let formats = [
        "%Y-%m-%d %H:%M",
        "%Y-%m-%d %H:%M:%S",
        "%Y/%m/%d %H:%M",
        "%Y/%m/%d %H:%M:%S",
        "%m/%d/%Y %H:%M",
        "%m/%d/%Y %H:%M:%S",
        "%d.%m.%Y %H:%M",
        "%d.%m.%Y %H:%M:%S",
    ];
    
    for format in formats.iter() {
        if let Ok(dt) = NaiveDateTime::parse_from_str(datetime_str, format) {
            return Some(dt);
        }
    }
    
    None
}