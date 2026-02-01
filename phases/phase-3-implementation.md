# Phase 3: Implementation

**Version Target:** v0.3.0
**Focus:** HTTP methods, async/sync variants, error handling

## Objectives

1. Implement HTTP methods for all endpoints
2. Provide async and sync variants
3. Implement proper error handling
4. Add streaming support (where applicable)

## Prerequisites

- Phase 1 completed (foundation)
- Phase 2 completed (primitives)
- All types defined and tested

## Implementation Order

Process endpoints by complexity:

1. **Simple endpoints** (GET, no body)
2. **Medium endpoints** (POST, simple body)
3. **Complex endpoints** (POST, streaming)

## HTTP Method Implementation

### Simple GET Endpoint
```rust
// src/http/endpoints/version.rs

impl Client {
    /// Get the API version.
    pub async fn version(&self) -> Result<VersionResponse> {
        let url = self.url("/api/version");

        let response = self.http()
            .get(&url)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Get the API version (blocking).
    pub fn version_sync(&self) -> Result<VersionResponse> {
        tokio::runtime::Runtime::new()
            .expect("Failed to create runtime")
            .block_on(self.version())
    }
}
```

### POST Endpoint
```rust
// src/http/endpoints/generate.rs

impl Client {
    /// Generate a completion.
    pub async fn generate(&self, request: GenerateRequest) -> Result<GenerateResponse> {
        let url = self.url("/api/generate");

        let response = self.http()
            .post(&url)
            .json(&request)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Generate a completion (blocking).
    pub fn generate_sync(&self, request: GenerateRequest) -> Result<GenerateResponse> {
        tokio::runtime::Runtime::new()
            .expect("Failed to create runtime")
            .block_on(self.generate(request))
    }
}
```

### Streaming Endpoint
```rust
// src/http/endpoints/generate.rs

use futures_util::Stream;
use tokio_stream::StreamExt;

impl Client {
    /// Generate a completion with streaming.
    pub async fn generate_stream(
        &self,
        request: GenerateRequest,
    ) -> Result<impl Stream<Item = Result<GenerateChunk>>> {
        let mut request = request;
        request.stream = Some(true);

        let url = self.url("/api/generate");

        let response = self.http()
            .post(&url)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(self.extract_error(response).await);
        }

        let stream = response
            .bytes_stream()
            .map(|result| {
                result
                    .map_err(LibraryError::from)
                    .and_then(|bytes| {
                        serde_json::from_slice(&bytes)
                            .map_err(LibraryError::from)
                    })
            });

        Ok(stream)
    }
}
```

## Error Handling

### Response Handler
```rust
// src/http/client.rs

impl Client {
    async fn handle_response<T: DeserializeOwned>(&self, response: reqwest::Response) -> Result<T> {
        let status = response.status();

        if status.is_success() {
            response.json().await.map_err(Into::into)
        } else {
            let error = self.extract_error(response).await;
            Err(error)
        }
    }

    async fn extract_error(&self, response: reqwest::Response) -> LibraryError {
        let status = response.status().as_u16();

        match response.json::<ErrorResponse>().await {
            Ok(err) => LibraryError::Api {
                status,
                message: err.error,
            },
            Err(_) => LibraryError::Api {
                status,
                message: format!("HTTP {}", status),
            },
        }
    }
}

#[derive(Debug, Deserialize)]
struct ErrorResponse {
    error: String,
}
```

### Specific Error Types
```rust
impl Client {
    pub async fn get_model(&self, name: &str) -> Result<Model> {
        let response = self.http()
            .get(&self.url(&format!("/api/models/{}", name)))
            .send()
            .await?;

        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            404 => Err(LibraryError::ModelNotFound(name.to_string())),
            status => Err(LibraryError::Api {
                status,
                message: self.extract_error_message(response).await,
            }),
        }
    }
}
```

## Module Organization

```
src/http/
├── mod.rs           # Module exports
├── client.rs        # Client struct and core methods
└── endpoints/
    ├── mod.rs       # Endpoint module exports
    ├── version.rs   # Version endpoint
    ├── generate.rs  # Generate endpoint
    ├── chat.rs      # Chat endpoint
    └── models.rs    # Model management endpoints
```

### Endpoint Module Pattern
```rust
// src/http/endpoints/mod.rs

mod version;
mod generate;
mod chat;
mod models;

// Methods are implemented directly on Client
// No additional exports needed
```

## Tasks Checklist

### Per Endpoint
- [ ] Write implementation plan
- [ ] Implement async method
- [ ] Implement sync method
- [ ] Add error handling
- [ ] Write unit tests
- [ ] Write integration test (optional)
- [ ] Create example

### Streaming Endpoints
- [ ] Implement stream method
- [ ] Handle partial JSON parsing
- [ ] Add stream consumption helpers
- [ ] Test stream behavior

### Quality
- [ ] All endpoints implemented
- [ ] Error handling consistent
- [ ] Tests pass
- [ ] Examples compile and run

## Testing Strategy

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, server_url};

    #[tokio::test]
    async fn test_generate_success() {
        let _m = mock("POST", "/api/generate")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"response": "Hello", "done": true}"#)
            .create();

        let client = Client::with_base_url(&server_url());
        let request = GenerateRequest::new("model", "Hi");

        let response = client.generate(request).await.unwrap();
        assert_eq!(response.response, "Hello");
    }

    #[tokio::test]
    async fn test_generate_error() {
        let _m = mock("POST", "/api/generate")
            .with_status(400)
            .with_body(r#"{"error": "Invalid model"}"#)
            .create();

        let client = Client::with_base_url(&server_url());
        let request = GenerateRequest::new("model", "Hi");

        let result = client.generate(request).await;
        assert!(matches!(result, Err(LibraryError::Api { status: 400, .. })));
    }
}
```

### Integration Tests
```rust
// tests/integration/api_tests.rs

#[tokio::test]
#[ignore] // Requires running service
async fn test_real_generate() {
    let client = Client::new();
    let request = GenerateRequest::new("llama3", "Say hello");

    let response = client.generate(request).await;
    assert!(response.is_ok());
}
```

## Examples

### Basic Example
```rust
// examples/generate_basic_async.rs

use library::{Client, GenerateRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let request = GenerateRequest::new("llama3", "Hello, world!");
    let response = client.generate(request).await?;

    println!("{}", response.response);
    Ok(())
}
```

### Streaming Example
```rust
// examples/generate_streaming.rs

use library::{Client, GenerateRequest};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let request = GenerateRequest::builder()
        .model("llama3")
        .prompt("Tell me a story")
        .build()?;

    let mut stream = client.generate_stream(request).await?;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        print!("{}", chunk.response);
    }

    println!();
    Ok(())
}
```

## Completion Criteria

1. All endpoints have async implementation
2. All endpoints have sync implementation
3. Streaming endpoints work correctly
4. Error handling is consistent
5. All tests pass
6. Examples compile and run

## Next Phase

After completing Phase 3, proceed to [Phase 4: Conveniences](phase-4-conveniences.md).
