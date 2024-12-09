# Perplexity AI API Client
[한국어 버전 (Korean version)](README_KO.md)

[English content here]

## Overview

This project provides a Rust client for interacting with the Perplexity AI API. It allows developers to easily integrate Perplexity AI's powerful language models into their applications for tasks such as text generation, question answering, and more.

## Features

- Simple and intuitive API for making requests to Perplexity AI
- Support for various Perplexity AI models
- Customizable request parameters including temperature and max tokens
- Role-based presets for optimized model selection and settings

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
pplx_client = "0.1.0"
```

## Usage

Here are examples of how to use the client:

### Basic Usage

```rust
use pplx_client::{PplxClient, Request, Message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = PplxClient::new("your_api_key_here");
    let messages = vec![
        Message::new("user", "Hello, how are you?"),
        Message::new("assistant", "I'm doing well, thank you! How can I assist you today?"),
    ];
    let request = Request::new(messages);
    
    let response = client.chat_completions(request).await?;
    println!("Response: {:?}", response);
    
    Ok(())
}
```

### Using Role-Based Presets

```rust
use pplx_client::{PplxClient, Request, Message, Role};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = PplxClient::new("your_api_key_here");
    let messages = vec![
        Message::new("user", "Write a creative story about a robot."),
    ];
    let request = Request::build(messages, Role::Creative)
        .max_tokens(200);
    
    let response = client.chat_completions(request).await?;
    println!("Response: {:?}", response);
    
    Ok(())
}
```

## Configuration

The `Request` struct allows for customization of the following parameters:

- `model`: Automatically set based on the chosen role or defaults to `SonarSmall`
- `messages`: A vector of `Message` structs representing the conversation
- `temperature`: Controls randomness (0.0 to 1.0), can be customized using `.temperature()`
- `max_tokens`: Optional maximum number of tokens to generate, set using `.max_tokens()`

## Role-Based Presets

The client supports the following roles, each with optimized model and temperature settings:

- `General`: Balanced for general queries
- `Code`: Precise for coding tasks
- `Finance`: Consistent for financial analysis
- `Creative`: More varied outputs for creative tasks
- `Science`: Balanced for scientific analysis

## Error Handling

The client returns a `Result` type. Make sure to handle potential errors appropriately in your application.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
