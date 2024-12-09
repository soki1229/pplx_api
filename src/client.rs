use reqwest::Client;
use crate::{
    error::PplxError,
    types::{Request, Response},
};
use log::{debug, error};

pub struct PplxClient {
    client: Client,
    api_key: String,
}

impl PplxClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn chat_completions(&self, request: Request) -> Result<Response, PplxError> {
        debug!("Sending request to API");
        let response = self.client
            .post("https://api.perplexity.ai/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;
    
        let status = response.status();
        debug!("Received response with status: {}", status);
    
        if status == reqwest::StatusCode::UNAUTHORIZED {
            error!("Authentication failed: Invalid API key");
            return Err(PplxError::AuthenticationError("Invalid API key".to_string()));
        }
        
        if !status.is_success() {
            let error_message = response.text().await?;
            error!("API error: {}", error_message);
            return Err(PplxError::ApiError(error_message));
        }
        
        debug!("Successfully received and parsed response");
        Ok(response.json().await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Message;

    #[tokio::test]
    async fn test_chat_completions() {
        env_logger::init();
        
        let client = PplxClient::new("".to_string());
        let request = Request::new(vec![
            Message::new("user", "Hello, how are you?"),
        ]);

        let response = client.chat_completions(request).await;

        assert!(matches!(response, Ok(_)));
    }
}