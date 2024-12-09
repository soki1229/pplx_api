use serde::Deserialize;
use super::{Choice, Usage};

#[derive(Deserialize)]
pub struct Response {
    pub id: String,
    pub model: String,
    pub object: String,
    pub created: i64,
    pub choices: Vec<Choice>,
    pub usage: Usage
}