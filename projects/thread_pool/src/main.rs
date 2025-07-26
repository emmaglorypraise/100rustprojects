use std::sync::{mpsc, Arc, Mutex};
use std::thread;
 
struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}
 
type Job = Box<dyn FnOnce() + Send + 'static>;
 
impl ThreadPool {
    fn new(size: usize) -> Self {
        assert!(size > 0, "Pool size must be greater than zero.");
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
 
        let workers = (0..size)
            .map(|id| Worker::new(id, Arc::clone(&receiver)))
            .collect();
 
        ThreadPool { workers, sender }
    }
 
    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(f)).unwrap();
    }
}
 
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
 
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(job) => {
                    println!("👷 Worker {} got a job!", id);
                    job();
                }
                Err(_) => {
                    println!("💤 Worker {} shutting down.", id);
                    break;
                }
            }
        });
 
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

fn main() {
    println!("🧵 Thread Pool Demo");
 
    let pool = ThreadPool::new(4);
 
    for i in 1..=8 {
        pool.execute(move || {
            println!("➡️ Task {} is running", i);
            std::thread::sleep(std::time::Duration::from_millis(500));
            println!("✅ Task {} done", i);
        });
    }
 
    std::thread::sleep(std::time::Duration::from_secs(3)); 
}
