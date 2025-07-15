# Project 013 – JSON Parser

## What I Built
A Rust CLI tool game to read and parse a JSON file, and print it in a clean, pretty format using serde_json.

## What I Learned

- Rust doesn't have native JSON support built-in, but the serde and serde_json crates make it powerful and ergonomic.

- serde_json::Value is flexible for parsing any valid JSON, whether it’s an object, array, string, or number.

- Using serde_json::to_string_pretty helps turn raw JSON into a clean, human-readable format — great for debugging or display.

- Everything in Rust returns a Result, so it's important to match on success and failure: Reading the file: fs::read_to_string(path), Parsing JSON: serde_json::from_str::<Value>(&content)

- Matching on Ok and Err allowed me to handle invalid JSON files or unreadable paths without crashing the program.

- Reading file paths from command-line arguments using env::args() taught me how Rust handles CLI input.

- I learned that the file path is not always relative to the code location — it's relative to where you run the program.

## Notes
How to Run:
``` 
cargo run sample.json

```

create a sample.json in your root folder with json like 
{
    "name": "Glory Praise",
    "age": 35,
    "languages": ["Rust", "Solidity", "Typescript", "JavaScript"]
}