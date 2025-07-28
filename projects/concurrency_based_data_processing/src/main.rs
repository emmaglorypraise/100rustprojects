use std::sync::{Arc, Mutex};
use std::thread;
 
fn main() {
    println!("🧮 Concurrent Data Processor");
 
    // Original data
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let data = Arc::new(data);
 
    // Shared output vector
    let results: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(Vec::new()));
 
    let mut handles = vec![];
 
    for chunk in data.chunks(2) {
        let chunk = chunk.to_vec(); // clone chunk to move into thread
        let results = Arc::clone(&results);
 
        let handle = thread::spawn(move || {
            let processed: Vec<i32> = chunk.iter().map(|n| n * n).collect(); // square each number
            let mut res = results.lock().unwrap();
            res.extend(processed);
        });
 
        handles.push(handle);
    }
 
    for handle in handles {
        handle.join().unwrap();
    }
 
    let final_results = results.lock().unwrap();
    println!("✅ Final processed results: {:?}", *final_results);
}