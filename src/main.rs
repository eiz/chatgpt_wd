use std::fs;

use anyhow::anyhow;
use clap::Parser;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use thirtyfour::prelude::*;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct ChatResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub choices: Vec<ChatChoice>,
    pub usage: ChatUsage,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct ChatChoice {
    pub index: u32,
    pub message: ChatMessage,
    pub finish_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct ChatUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    sys: Option<String>,
    url: String,
}

async fn do_one_element(
    openai_key: String,
    driver: WebDriver,
    mut query: ChatRequest,
    el: WebElement,
    text: String,
) -> anyhow::Result<()> {
    query.messages.push(ChatMessage {
        role: "user".to_owned(),
        content: text.to_owned(),
    });

    let body = serde_json::to_string(&query)?;
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(&openai_key)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await?;
    let status = response.status();
    let response_text = response.text().await?;

    if !status.is_success() {
        eprintln!("uh oh, an error: \n{}", response_text);
    }

    let result: ChatResponse = serde_json::from_str(&response_text)?;
    let content = &result.choices[0].message.content;

    driver
        .execute(
            "arguments[0].innerText = arguments[1]",
            vec![el.to_json()?, serde_json::to_value(&content)?],
        )
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let openai_key = fs::read_to_string(
        dirs::home_dir()
            .ok_or_else(|| anyhow!["missing $HOME"])?
            .join(".openai"),
    )?
    .trim()
    .to_owned();
    let sys = args
        .sys
        .unwrap_or("You rewrite text as if it was written by a pirate. Arr, matey! Each response is the text, but piratey.".to_owned());
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    driver.goto(&args.url).await?;
    let elements = driver.find_all(By::XPath("//*[text() and not(*)]")).await?;

    let query_template = ChatRequest {
        model: "gpt-3.5-turbo".to_owned(),
        temperature: Some(1.0),
        messages: vec![ChatMessage {
            role: "system".to_owned(),
            content: sys,
        }],
        ..Default::default()
    };

    let mut to_rewrite = vec![];
    for el in elements {
        let text = el.text().await?;

        if text.len() > 20 {
            println!("{}", text);
            to_rewrite.push((el.clone(), text));
        }
    }

    let handles = {
        let driver = driver.clone();
        let handles = tokio_stream::iter(to_rewrite.into_iter().map(move |(el, text)| {
            let query = query_template.clone();
            let openai_key = openai_key.clone();
            let driver = driver.clone();
            do_one_element(openai_key, driver, query, el, text)
        }))
        .buffer_unordered(8);
        handles
    };

    let _ = handles.collect::<Vec<anyhow::Result<()>>>().await;
    Ok(())
}
