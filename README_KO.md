# Perplexity AI API 클라이언트

## 개요

이 프로젝트는 Perplexity AI API와 상호 작용하기 위한 Rust 클라이언트를 제공합니다. 개발자가 텍스트 생성, 질문 답변 등의 작업을 위해 Perplexity AI의 강력한 언어 모델을 애플리케이션에 쉽게 통합할 수 있게 해줍니다.

## 기능

- Perplexity AI에 요청을 보내기 위한 간단하고 직관적인 API
- 다양한 Perplexity AI 모델 지원
- 온도 및 최대 토큰 수 등 사용자 정의 가능한 요청 매개변수
- 최적화된 모델 선택 및 설정을 위한 역할 기반 프리셋

## 설치

`Cargo.toml`에 다음을 추가하세요:

```toml
[dependencies]
pplx_client = "0.1.0"
```

## 사용법

클라이언트 사용 예시입니다:

### 기본 사용법

```rust
use pplx_client::{PplxClient, Request, Message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = PplxClient::new("your_api_key_here");
    let messages = vec![
        Message::new("user", "안녕하세요, 어떻게 지내세요?"),
        Message::new("assistant", "잘 지내고 있습니다, 감사합니다! 오늘 어떻게 도와드릴까요?"),
    ];
    let request = Request::new(messages);
    
    let response = client.chat_completions(request).await?;
    println!("응답: {:?}", response);
    
    Ok(())
}
```

### 역할 기반 프리셋 사용

```rust
use pplx_client::{PplxClient, Request, Message, Role};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = PplxClient::new("your_api_key_here");
    let messages = vec![
        Message::new("user", "로봇에 대한 창의적인 이야기를 써주세요."),
    ];
    let request = Request::build(messages, Role::Creative)
        .max_tokens(200);
    
    let response = client.chat_completions(request).await?;
    println!("응답: {:?}", response);
    
    Ok(())
}
```

## 설정

`Request` 구조체는 다음 매개변수의 사용자 정의를 허용합니다:

- `model`: 선택한 역할에 따라 자동으로 설정되거나 기본값으로 `SonarSmall` 사용
- `messages`: 대화를 나타내는 `Message` 구조체의 벡터
- `temperature`: 무작위성을 제어 (0.0에서 1.0 사이), `.temperature()`를 사용하여 사용자 정의 가능
- `max_tokens`: 생성할 최대 토큰 수 (선택 사항), `.max_tokens()`를 사용하여 설정

## 역할 기반 프리셋

클라이언트는 각각 최적화된 모델 및 온도 설정을 가진 다음 역할들을 지원합니다:

- `General`: 일반적인 쿼리에 균형 잡힌 설정
- `Code`: 코딩 작업을 위한 정밀한 설정
- `Finance`: 재무 분석을 위한 일관된 설정
- `Creative`: 창의적인 작업을 위한 다양한 출력 설정
- `Science`: 과학적 분석을 위한 균형 잡힌 설정

## 오류 처리

클라이언트는 `Result` 타입을 반환합니다. 애플리케이션에서 잠재적인 오류를 적절히 처리하도록 하세요.

## 기여

기여는 언제나 환영합니다! Pull Request를 자유롭게 제출해 주세요.

## 라이선스

이 프로젝트는 MIT 라이선스 하에 있습니다 - 자세한 내용은 LICENSE 파일을 참조하세요.