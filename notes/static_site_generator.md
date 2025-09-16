# Project 089 – Static Site Generator (Markdown → HTML)

## What I Built
A CLI tool that reads .md files from a directory and generates a static website with .html files using templates.Using `pulldown-cmark` for parsing Markdown and basic templating to render full HTML pages.

The tool reads all `.md` files from the `content/` folder and write `.html` versions into `public/`.

## What I Learned


## Notes
### Run the App
```
cargo run
```

### Key Concepts:
- Use walkdir to scan folders recursively

- Parse Markdown into HTML using pulldown-cmark

- Write full .html pages with embedded content

- Separate logic for Markdown → HTML and Template rendering

We now have a basic static site generator, the foundation for a custom blog engine, documentation site, or developer portfolio.