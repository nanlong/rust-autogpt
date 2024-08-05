use crate::models::general::llm::{APIResponse, ChatCompletion, Message};
use dotenvy::dotenv;
use reqwest::{header::HeaderMap, Client};
use std::env;

// Call Large Language Model (i.e. GPT-4)
pub async fn call_gpt(messages: Vec<Message>) -> anyhow::Result<String> {
    dotenv().expect("Failed to load .env file");

    let url = env::var("OPENAI_URL").expect("OPENAI_URL not found in environment variables");
    let api_key = env::var("OPENAI_KEY").expect("OPENAI_KEY not found in environment variables");
    let model = env::var("OPENAI_MODEL").expect("OPENAI_MODEL not found in environment variables");

    let mut headers = HeaderMap::new();

    // Create api key header
    headers.insert("Authorization", format!("Bearer {}", api_key).parse()?);

    // Create client
    let client = Client::builder().default_headers(headers).build()?;

    // Create chat completion
    let chat_completion = ChatCompletion {
        model,
        messages,
        temperature: 0.1,
    };

    // Troubleshooting
    let response = client
        .post(&url)
        .json(&chat_completion)
        .send()
        .await?
        .json::<APIResponse>()
        .await?;

    Ok(response.choices[0].message.content.clone())
}
