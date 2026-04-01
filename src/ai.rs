use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::io::{self, ErrorKind};

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
struct ChatResponse {
    message: ResponseMessage,
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

        let chat_response: ChatResponse = response
            .json()
            .await
            .map_err(|e| io::Error::new(ErrorKind::InvalidData, e))?;

        Ok(chat_response.message.content)
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
