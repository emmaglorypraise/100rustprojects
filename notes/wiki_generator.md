# Project 093 – Browser Automation with Fantoccini (WebDriver Client in Rust)

## What I Built
A Rust CLI  tool that scans a folder of .md files, builds a navigable HTML wiki, and generates an index of links. Combining directory walking, Markdown rendering, and template generation. 


## What I Learned

## Notes
### Run the Generator
```
cargo run
```
Then open public/index.html in your browser 📂

### Example Output
```
📘 My Wiki
 
• [intro.html](intro.html)  
• [topics/rust.html](topics/rust.html)  
• [topics/async.html](topics/async.html)
```

### Key Concepts:
- `WalkDir` to recursively find Markdown files

- Convert `.md` to `.html` with `pulldown-cmark`

- Generate internal HTML links for a full index

You now have a personal offline Markdown wiki—great for documentation, knowledge bases, or local publishing.