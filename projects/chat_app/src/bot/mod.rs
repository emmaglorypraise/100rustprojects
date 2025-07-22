mod history;
mod responses;

use crate::bot::history::ChatHistory;
use crate::bot::responses::RESPONSES;
use chrono::Local;
use std::collections::HashMap;

pub struct ChatBot {
    pub history: ChatHistory,
    responses: HashMap<String, String>,
}

impl ChatBot {
    pub fn new() -> Self {
        ChatBot {
            history: ChatHistory::new(),
            responses: RESPONSES.clone(),
        }
    }

    pub fn process_message(&mut self, message: &str) -> String {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    // Add user message to history
    self.history.add_entry(&timestamp, "You", message);

    let msg = message.to_lowercase();
    // let words: Vec<&str> = msg.split_whitespace().collect();
    
    let mut best_score = 0;
    let mut best_response = None;
    
    for (keys, response) in &self.responses {
        let current_score = keys.split('|')
            .map(|pattern| {
                let pattern_lower = pattern.to_lowercase();
                if !msg.contains(&pattern_lower) {
                    return 0;
                }
                // Score based on:
                // 1. Length of matching pattern (longer = better)
                // 2. Number of matching words
                pattern.split_whitespace().count() + pattern.len()
            })
            .max()
            .unwrap_or(0);
        
        if current_score > best_score {
            best_score = current_score;
            best_response = Some(response);
        }
    }

    // Get the response text to store and return
    let response_text = best_response
        .map(|r| r.as_str())
        .unwrap_or("I'm not sure how to respond to that.");
    
    // Add bot response to history
    self.history.add_entry(&timestamp, "Bot", response_text);
    
    response_text.to_string()
  }

    
    // pub fn process_message(&mut self, message: &str) -> String {
    //     let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        
        // // Add user message to history
        // self.history.add_entry(&timestamp, "You", message);

    //     // Generate response
    //     let msg = message.to_lowercase();
    //      let response = self.responses.iter()
    //         .find(|(keys, _)| {
    //             keys.split('|').any(|k| msg.contains(&k.trim().to_lowercase()))
    //         })
    //         .map(|(_, v)| v.clone())
    //         .unwrap_or_else(|| {
    //             // Fallback: check individual words
    //             message.split_whitespace()
    //                 .find_map(|word| self.responses.get(&word.to_lowercase()))
    //                 .cloned()
    //                 .unwrap_or_else(|| "I'm not sure how to respond to that.".to_string())
    //         });

    //     // Add bot response to history
    //     self.history.add_entry(&timestamp, "Bot", &response);

    //     response
    // }


}