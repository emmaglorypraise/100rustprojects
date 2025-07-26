use reqwest::blocking::get;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

fn main() {
    println!("🌐 Parallel Web Crawler");

    let urls = vec![
        "https://www.rust-lang.org".to_string(),
        "https://www.google.com".to_string(),
        "https://docs.rs".to_string(),
        "https://httpbin.org/delay/1".to_string(),
        "https://thisurldoesnotexist.com".to_string(), // intentionally fails
    ];

    let pool_size = 4;
    let (tx, rx) = mpsc::channel::<String>(); // Explicit String type
    let rx = Arc::new(Mutex::new(rx));
    let mut handles = vec![];

    // Create worker threads
    for i in 0..pool_size {
        let rx = Arc::clone(&rx);
        let handle = thread::spawn(move || loop {
            let url = match rx.lock().unwrap().recv() {
                Ok(u) => u,
                Err(_) => break,
            };
            println!("🧵 Worker {} fetching: {}", i, url);
            match fetch_url(&url) {
                Ok(status) => println!("✅ {} => {}", url, status),
                Err(e) => println!("❌ {} => {}", url, e),
            }
        });
        handles.push(handle);
    }

    // Send URLs to workers
    for url in urls {
        tx.send(url).unwrap();
    }

    // Explicitly drop the transmitter to close the channel
    drop(tx);

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap_or_else(|e| {
            eprintln!("⚠️ Thread panicked: {:?}", e);
        });
    }

    println!("🏁 All workers finished");
}

fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
    let response = get(url)?;
    Ok(format!("Status: {}", response.status()))
}