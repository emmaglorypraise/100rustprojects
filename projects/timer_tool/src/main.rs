use std::{io, thread, time::Duration};
use std::io::{Write};
use std::sync::mpsc;
use std::thread::sleep;

fn main() {
    println!("⏳ Basic Timer Tool");
    println!("Enter the time duration (format: hours, minutes, seconds)");

    let duration =  match get_timer_input() {
        Some(dur) => dur,
        None => {
            println!("❌ Invalid input. Please enter numbers only (e.g., 0 1 30 for one minutes thirty seconds).");
            return;
        }
    };

    println!("✅ Timer set for: {} hours, {} minutes, {} seconds", duration.0, duration.1, duration.2);

    println!("🔘 Press 'p' + Enter to pause, 'r' + Enter to resume.");

    start_timer(duration.0, duration.1, duration.2);

    println!("🎉 Times up!");
}

fn get_timer_input() -> Option<(u64, u64, u64)> {
    let mut input = String::new();
    io::stdin()
      .read_line(&mut input)
      .expect("Failed to get input.");

    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() != 3 {
        return None;
    }

    let hours = parts[0].parse::<u64>().ok()?;
    let minutes = parts[1].parse::<u64>().ok()?;
    let seconds = parts[2].parse::<u64>().ok()?;

    Some((hours, minutes, seconds))
}

// fn start_timer(hours: u64, minutes:u64, seconds:u64) {
//     let total_seconds = hours * 3600 + minutes * 60 + seconds;
//     for i in (1..=total_seconds).rev() {
//         let hrs = i/3600;
//         let mins = (i % 3600) / 60;
//         let secs = i % 60;

//         print!("\r⏲️ Time Remaining: {:02}:{:02}:{:02}", hrs, mins, secs);
//         io::stdout().flush().unwrap();
//         thread::sleep(Duration::from_secs(1));
//     }
//        println!();
// }

fn start_timer(hours: u64, minutes: u64, seconds: u64) {
    let total_seconds = hours * 3600 + minutes * 60 + seconds;
    let (tx, rx) = mpsc::channel::<char>();

    thread::spawn(move || {
        loop {
            let mut cmd = String::new();
            if io::stdin().read_line(&mut cmd).is_ok() {
                if let Some(c) = cmd.trim().chars().next() {
                    if c == 'p' || c == 'r' {
                        let _ = tx.send(c);
                    }
                }
            }
        }
    });

    let mut remaining = total_seconds;
    let mut paused = false;

    while remaining > 0 {
        if let Ok(cmd) = rx.try_recv() {
            match cmd {
                'p' => {
                    paused = true;
                    println!("\n⏸️ Paused at {:02}:{:02}:{:02}", remaining / 3600, (remaining % 3600) / 60, remaining % 60);
                }
                'r' => {
                    if paused {
                        paused = false;
                        println!("▶️ Resumed");
                    }
                }
                _ => {}
            }
        }

        if paused {
            sleep(Duration::from_millis(200));
            continue;
        }

        let hrs = remaining / 3600;
        let mins = (remaining % 3600) / 60;
        let secs = remaining % 60;
        print!("\r⏲️ Time Remaining: {:02}:{:02}:{:02}", hrs, mins, secs);
        io::stdout().flush().unwrap();

        sleep(Duration::from_secs(1));
        remaining -= 1;
    }

    println!();
}






