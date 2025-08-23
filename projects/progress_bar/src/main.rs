use indicatif::{ProgressBar, ProgressStyle};
use std::{thread, time::Duration};
 
fn main() {
    println!("🚀 Starting simulated task...");
 
    let bar = ProgressBar::new(100);
    bar.set_style(
        ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>3}/{len} {msg}")
            .unwrap()
            .progress_chars("=>-"),
    );
 
    for i in 0..100 {
        bar.set_message(format!("step {}", i));
        bar.inc(1);
        thread::sleep(Duration::from_millis(50));
    }
 
    bar.finish_with_message("✅ Done!");
}