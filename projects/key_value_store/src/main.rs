use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, Write};
use serde::{Deserialize, Serialize};

const STORE_FILE: &str = "kv_store.json";

#[derive(Serialize, Deserialize)]
struct Store {
    data: HashMap<String, String>,
}

impl Store {
    fn new() -> Self {
        if let Ok(file) = File::open(STORE_FILE) {
            let reader = BufReader::new(file);
            if let Ok(store) = serde_json::from_reader(reader) {
                return store;
            }
        }
        Store { data: HashMap::new() }
    }

    fn save(&self) {
        if let Ok(mut file) = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(STORE_FILE)
        {
            serde_json::to_writer_pretty(&mut file, self).unwrap();
        }
    }

    fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
        self.save();
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    fn delete(&mut self, key: &str) {
        self.data.remove(key);
        self.save();
    }

    fn list(&self) {
        println!("📦 Stored Keys:");
        for key in self.data.keys() {
            println!("- {}", key);
        }
    }
}

fn main() {
    println!("🗃️ Key-Value Store (set/get/delete/list/exit)");
    let mut store = Store::new();

    loop {
        let cmd = prompt("\n> ");
        let parts: Vec<&str> = cmd.trim().splitn(3, ' ').collect();

        match parts.as_slice() {
            ["set", key, value] => {
                store.set(key.to_string(), value.to_string());
                println!("✅ Key set.");
            }
            ["get", key] => match store.get(key) {
                Some(value) => println!("🔑 Value: {}", value),
                None => println!("❌ Key not found."),
            },
            ["delete", key] => {
                store.delete(key);
                println!("🗑️ Key deleted (if it existed).");
            }
            ["list"] => store.list(),
            ["exit"] => {
                println!("👋 Exiting.");
                break;
            }
            _ => println!("❌ Invalid command. Use: set/get/delete/list/exit"),
        }
    }
}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}