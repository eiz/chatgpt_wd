use std::fs;

use anyhow::anyhow;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
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

#[derive(Serialize, Deserialize, Debug, Default)]
struct ChatResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub choices: Vec<ChatChoice>,
    pub usage: ChatUsage,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct ChatChoice {
    pub index: u32,
    pub message: ChatMessage,
    pub finish_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
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
    query: Vec<String>,
}

fn leading_spaces(s: &str) -> usize {
    let mut count = 0;
    for c in s.chars() {
        if c.is_whitespace() {
            count += 1;
        } else {
            break;
        }
    }
    count
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
    let sys = args.sys.unwrap_or(
        "Assistant answers all queries in the form of a complete python script. You may only respond with valid python code, no idle statements.".to_owned(),
    );
    let query = ChatRequest {
        model: "gpt-3.5-turbo".to_owned(),
        temperature: Some(0.0),
        messages: vec![
            ChatMessage {
                role: "system".to_owned(),
                content: sys,
            },
            ChatMessage {
                role: "user".to_owned(),
                content: args.query.join(" "),
            },
            ChatMessage {
                role: "assistant".to_owned(),
                content: "#!/usr/bin/env python3".to_owned(),
            },
        ],
        ..Default::default()
    };
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
        std::process::exit(1);
    }

    let result: ChatResponse = serde_json::from_str(&response_text)?;
    let mut in_code = false;
    let mut last_indent = 0;
    let content = &result.choices[0].message.content;
    let strip_markdown = content.contains("```");

    for line in content.lines() {
        if strip_markdown && line.starts_with("```") {
            in_code = !in_code;
            if !in_code {
                break;
            }
            continue;
        }
        if !strip_markdown || in_code {
            println!("{}", line);
            last_indent = leading_spaces(line);
        } else {
            for comment_line in textwrap::wrap(line, (80 - last_indent).max(20)) {
                println!("{}# {}", " ".repeat(last_indent), comment_line);
            }
        }
    }
    Ok(())
}
