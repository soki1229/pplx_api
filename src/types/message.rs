use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String
}

#[derive(Serialize, Deserialize)]
pub struct Choice {
    index: i32,
    finish_reason: String,
    message: Message
}

#[derive(Serialize, Deserialize)]
pub struct Usage {
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32
}

impl Message {
    pub fn new(role: &str, content: &str) -> Self {
        Self {
            role: role.to_string(),
            content: content.to_string(),
        }
    }
}