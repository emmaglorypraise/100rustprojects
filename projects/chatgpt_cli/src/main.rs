use dotenvy::dotenv;
use std::env;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

#[derive(Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: Vec<Message<'a>>,
    temperature: f32,
}

#[derive(Serialize)]
struct Message<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Deserialize, Debug)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: MessageContent,
}

#[derive(Deserialize, Debug)]
struct MessageContent {
    content: String,
}

#[derive(Deserialize, Debug)]
struct ApiError {
    error: ErrorDetail,
}

#[derive(Deserialize, Debug)]
struct ErrorDetail {
    message: String,
    #[serde(rename = "type")]
    error_type: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY")?;

    print!("💬 You: ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let user_input = input.trim();

    let req_body = ChatRequest {
        model: "gpt-3.5-turbo",
        messages: vec![Message {
            role: "user",
            content: user_input,
        }],
        temperature: 0.7,
    };

    let client = Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(&api_key)
        .json(&req_body)
        .send()
        .await?;

    // Get the response text first so we can use it for both success and error cases
    let response_text = response.text().await?;

    // Try to parse as successful response first
    match serde_json::from_str::<ChatResponse>(&response_text) {
        Ok(res) => {
            if let Some(choice) = res.choices.first() {
                println!("🤖 Assistant: {}", choice.message.content.trim());
            } else {
                println!("❌ No response from assistant");
            }
        }
        Err(_) => {
            // If parsing as ChatResponse failed, try to parse as error
            match serde_json::from_str::<ApiError>(&response_text) {
                Ok(api_error) => {
                    eprintln!("❌ API Error ({}): {}", api_error.error.error_type, api_error.error.message);
                }
                Err(e) => {
                    eprintln!("❌ Failed to parse response as either success or error: {}", e);
                    eprintln!("Raw response: {}", response_text);
                }
            }
        }
    }

    Ok(())
}