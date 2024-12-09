use serde::Serialize;
use crate::models::{Model, Role};
use super::Message;

#[derive(Serialize)]
pub struct Request {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: Option<i32>
}

impl Request {
    pub fn new(messages: Vec<Message>) -> Self {
        Self {
            model: String::from(Model::SonarSmall.as_str()),
            messages,
            temperature: 0.7,
            max_tokens: None
        }
    }

    pub fn build(messages: Vec<Message>, role: Role) -> Self {
        let (model, temperature) = Self::preset(role);
        Self {
            model: String::from(model.as_str()),
            messages,
            temperature,
            max_tokens: None
        }
    }

    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature;
        self
    }
    
    pub fn max_tokens(mut self, max_tokens: i32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    fn preset(role: Role) -> (Model, f32) {        
        match role {
            Role::General => (
                Model::SonarLarge,  // Balanced for general queries
                0.7
            ),
            Role::Code => (
                Model::SonarLarge,  // Precise for coding tasks
                0.2
            ),
            Role::Finance => (
                Model::SonarLarge,  // Consistent for financial analysis
                0.1
            ),
            Role::Creative => (
                Model::SonarLarge,  // More varied outputs
                0.9
            ),
            Role::Science => (
                Model::SonarLarge,  // Balanced for scientific analysis
                0.3
            )
        }
    }

}
