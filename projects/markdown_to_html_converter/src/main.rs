use pulldown_cmark::{Parser, Options, html};
use std::env;
use std::fs;
 
fn main() {
    let args: Vec<String> = env::args().collect();
 
    if args.len() != 3 {
        eprintln!("Usage: {} <input.md> <output.html>", args[0]);
        return;
    }
 
    let input_file = &args[1];
    let output_file = &args[2];
 
    let markdown_input = fs::read_to_string(input_file)
        .expect("❌ Failed to read input file");
 
    let parser = Parser::new_ext(&markdown_input, Options::all());
 
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
 
    fs::write(output_file, html_output).expect("❌ Failed to write output file");
 
    println!("✅ Markdown converted to HTML and saved to {}", output_file);
}
