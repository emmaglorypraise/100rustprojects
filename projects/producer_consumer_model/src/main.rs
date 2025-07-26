use std::sync::mpsc::{self, SyncSender};
use std::thread;
use std::time::Duration;
 
fn main() {
    println!("🔄 Producer-Consumer Demo");
 
    let (tx, rx) = mpsc::sync_channel(3); // bounded buffer size = 3
 
    // Producer thread
    let producer = thread::spawn(move || {
        for i in 1..=10 {
            println!("📤 Producing task {}", i);
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(300)); // simulate work
        }
        println!("✅ Producer finished.");
    });
 
    // Consumer thread
    let consumer = thread::spawn(move || {
        while let Ok(task) = rx.recv() {
            println!("📥 Consuming task {}", task);
            thread::sleep(Duration::from_millis(500)); // simulate slower work
        }
    });
 
    producer.join().unwrap();
    thread::sleep(Duration::from_secs(2)); // allow consumer to finish
    println!("👋 All tasks processed.");
}