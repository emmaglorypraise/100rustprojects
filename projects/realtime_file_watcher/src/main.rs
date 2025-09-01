use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher, EventKind};
use std::sync::mpsc::channel;
use std::env;
 
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file-or-directory-to-watch>", args[0]);
        return;
    }
 
    let path = &args[1];
 
    println!("👁️ Watching for changes in: {}", path);
 
    let (tx, rx) = channel();
 
     let mut watcher: RecommendedWatcher = Watcher::new(tx, Config::default()).unwrap();
     if let Err(e) = watcher.watch(path.as_ref(), RecursiveMode::Recursive) {
            eprintln!("Failed to watch path \"{}\": {:?}", path, e);
            return;
     }
 
    loop {
        match rx.recv() {
            Ok(event) => match event {
                Ok(e) => {
                    if let EventKind::Modify(_) = e.kind {
                        println!("📝 File modified: {:?}", e.paths);
                    } else {
                        println!("📦 Event: {:?}", e);
                    }
                }
                Err(e) => eprintln!("❌ Watch error: {:?}", e),
            },
            Err(_) => break,
        }
    }
}