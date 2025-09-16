# Project 092 – Browser Automation with Fantoccini (WebDriver Client in Rust)

## What I Built
A Rust CLI app that uses fantoccini to automate a browser (headless or visible). Launch a WebDriver session, visit a page, and interact with elements — ideal for web scraping, testing, or automation workflows.

## What I Learned


## Notes
Before running the app, install and run a WebDriver instance:

###  Chrome:
# Install ChromeDriver
```
brew install chromedriver
 ```
# Start it
```
chromedriver
Default runs at: http://localhost:9515
```
### For Firefox:
```
geckodriver
```

#### Make sure your WebDriver is running before executing the program:
```
# For ChromeDriver
chromedriver --port=9515

# Or for GeckoDriver (Firefox)
geckodriver --port 9515
```

### How to Run the Application:
```
cargo run
```

### Key Concepts:
- `fantoccini` is a WebDriver client in Rust (like Selenium)

- Use `Locator` to find page elements (by text, id, tag, etc.)

- Supports actions: click, type, wait, screenshot, navigate

You’ve now built a browser automation tool in Rust—great for testing, scraping, monitoring, or task automation.





