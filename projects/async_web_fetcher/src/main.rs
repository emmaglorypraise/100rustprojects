use reqwest::Error;
 
#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://www.rust-lang.org";
    println!("🌐 Fetching: {}", url);
 
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
 
    println!("✅ Response received ({} bytes)", body.len());
    println!("\nPreview:\n{}", &body[..200.min(body.len())]); // Show first 200 chars
 
    Ok(())
}