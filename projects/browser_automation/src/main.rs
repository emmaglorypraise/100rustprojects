use fantoccini::{ClientBuilder, Locator};
use tokio;

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    println!("🕸️ Launching browser automation...");

    // Connect to WebDriver using the recommended ClientBuilder
    let mut client = ClientBuilder::native()
        .connect("http://localhost:9515")
        .await
        .expect("❌ Failed to connect to WebDriver. Make sure ChromeDriver/GeckoDriver is running");

    // Navigate to Rust homepage
    client.goto("https://www.rust-lang.org").await?;

    // Get and display page title
    let title = client.title().await?;
    println!("📄 Page Title: {}", title);

    // Try to find and click the "Learn" link
    match client.find(Locator::LinkText("Learn")).await {
        Ok(link) => {
            println!("🔗 Clicking 'Learn' link...");
            link.click().await?;
        }
        Err(e) => {
            println!("⚠️  'Learn' link not found: {}", e);
        }
    }

    // Take and save screenshot
    let screenshot = client.screenshot().await?;
    std::fs::write("screenshot.png", screenshot)
        .expect("❌ Failed to save screenshot");
    println!("📸 Screenshot saved to screenshot.png");

    // Close the browser session
    client.close().await?;

    Ok(())
}