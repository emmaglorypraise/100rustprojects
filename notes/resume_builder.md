# Project 096 – Resume Builder (TOML to PDF via HTML)

## What I Built
A Rust CLI tool that reads resume data from a TOML file, renders it into an HTML template, and optionally converts it into a PDF using an external tool like `WeasyPrint`. Practice TOML parsing, template injection, and PDF generation hooks.


## What I Learned
- Conflict resolution: patch overwrites base

## Notes
### Sample resume.toml
```
name = "Ada Lovelace"
title = "Mathematician & Programmer"
email = "ada@analytical.engine"
phone = "+44 123 4567"
 
[[experience]]
company = "Analytical Engine Research"
role = "Lead Algorithm Architect"
year = "1843"
 
[[experience]]
company = "Royal Society"
role = "Scientific Advisor"
year = "1850"
```

### Run the App
```
cargo run 
Optional: Install WeasyPrint for PDF support.
```

### Output:
- `resume.html`: A clean, structured HTML resume

- `resume.pdf`: PDF copy (if WeasyPrint is installed)

### Install WeasyPrint:
```
# On macOS
brew install weasyprint

# On Ubuntu/Debian
sudo apt-get install weasyprint

# On Windows (requires Python)
pip install weasyprint
```

### Usage:
Create your resume.toml file with all your resume data

1. Install WeasyPrint:
```
macOS: brew install weasyprint

Ubuntu/Debian: sudo apt-get install weasyprint

Windows: pip install weasyprint
```
2. Run the program: cargo run

You'll get both resume.html and resume.pdf files