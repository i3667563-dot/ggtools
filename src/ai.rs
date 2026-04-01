use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::io::{self, BufRead, ErrorKind};

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct StreamResponse {
    message: ResponseMessage,
    #[serde(default)]
    done: bool,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

pub struct Ollama {
    client: Client,
    url: String,
}

impl Ollama {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            url: "http://localhost:11434".to_string(),
        }
    }

    pub fn with_url(url: &str) -> Self {
        Self {
            client: Client::new(),
            url: url.to_string(),
        }
    }

    pub async fn prompt(&self, model: &str, message: &str) -> Result<String, io::Error> {
        let mut full_response = String::new();
        self.prompt_stream(model, message, |chunk| {
            full_response.push_str(&chunk);
        }).await?;
        Ok(full_response)
    }

    pub async fn prompt_stream<F>(&self, model: &str, message: &str, mut callback: F) -> Result<(), io::Error>
    where
        F: FnMut(String) + Send,
    {
        let request = ChatRequest {
            model: model.to_string(),
            messages: vec![Message {
                role: "user".to_string(),
                content: message.to_string(),
            }],
        };

        let response = self.client
            .post(format!("{}/api/chat", self.url))
            .json(&request)
            .send()
            .await
            .map_err(|e| io::Error::new(ErrorKind::ConnectionRefused, e))?;

        if !response.status().is_success() {
            return Err(io::Error::new(ErrorKind::Other, "Request failed"));
        }

        let body = response
            .text()
            .await
            .map_err(|e| io::Error::new(ErrorKind::InvalidData, e))?;

        for line in body.lines() {
            if let Ok(stream_resp) = serde_json::from_str::<StreamResponse>(line) {
                callback(stream_resp.message.content);
            }
        }

        Ok(())
    }
}

impl Default for Ollama {
    fn default() -> Self {
        Self::new()
    }
}

pub fn prompt(model: &str, message: &str) -> Result<String, io::Error> {
    let ollama = Ollama::new();
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(ollama.prompt(model, message))
}

pub fn prompt_to(url: &str, model: &str, message: &str) -> Result<String, io::Error> {
    let ollama = Ollama::with_url(url);
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(ollama.prompt(model, message))
}

pub fn prompt_stream<F>(model: &str, message: &str, callback: F) -> Result<(), io::Error>
where
    F: FnMut(String) + Send + 'static,
{
    let ollama = Ollama::new();
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(ollama.prompt_stream(model, message, callback))
}

pub fn prompt_stream_to<F>(url: &str, model: &str, message: &str, callback: F) -> Result<(), io::Error>
where
    F: FnMut(String) + Send + 'static,
{
    let ollama = Ollama::with_url(url);
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(ollama.prompt_stream(model, message, callback))
}
