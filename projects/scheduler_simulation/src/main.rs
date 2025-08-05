use std::thread;
use std::time::{Duration, Instant};
 
#[derive(Debug)]
struct Task {
    name: String,
    delay_secs: u64,
}
 
fn main() {
    println!("🕒 Task Scheduler Simulation");
 
    let mut tasks = vec![
        Task { name: "Backup".into(), delay_secs: 3 },
        Task { name: "Clean Temp Files".into(), delay_secs: 1 },
        Task { name: "Send Report".into(), delay_secs: 2 },
    ];
 
    tasks.sort_by_key(|t| t.delay_secs); // optional: shortest job first
 
    let start = Instant::now();
 
    for task in tasks {
        let wait = Duration::from_secs(task.delay_secs);
        println!("⏳ Waiting {}s to run '{}'", task.delay_secs, task.name);
        thread::sleep(wait);
        let elapsed = Instant::now().duration_since(start).as_secs();
        println!("✅ [{}s] Task '{}' completed", elapsed, task.name);
    }
 
    println!("🎯 All tasks completed.");
}