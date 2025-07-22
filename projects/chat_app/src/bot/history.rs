use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io;
use std::path::Path;



#[derive(Serialize, Deserialize)]
struct ChatEntry {
    timestamp: String,
    user: String,
    message: String,
}


#[derive(Serialize, Deserialize)]
pub struct ChatHistory {
    entries: Vec<ChatEntry>,
}

impl ChatHistory {
    pub fn new() -> Self {
        ChatHistory { 
          entries: Vec::new() 
        }
    }

    pub fn add_entry(&mut self, timestamp: &str, user: &str, message: &str) {
        self.entries.push(ChatEntry {
            timestamp: timestamp.to_string(),
            user: user.to_string(),
            message: message.to_string(),
        });
    }

    pub fn load(&mut self) -> io::Result<()> {
        if Path::new("chat_history.json").exists() {
            let file = File::open("chat_history.json")?;
            *self = serde_json::from_reader(file).unwrap_or_else(|_| ChatHistory::new());
        }
        Ok(())
    }

    pub fn save(&self) -> io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("chat_history.json")?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }

    pub fn print(&self) {
        println!("\n🗒️ Chat History:");
        for entry in &self.entries {
            println!("[{}] {}: {}", entry.timestamp, entry.user, entry.message);
        }
    }

    pub fn clear(&mut self) -> io::Result<()> {
        self.entries.clear();
        if Path::new("chat_history.json").exists() {
            std::fs::remove_file("chat_history.json")?;
        }
        Ok(())
    }
}