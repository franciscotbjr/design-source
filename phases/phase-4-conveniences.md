# Phase 4: Conveniences

**Version Target:** v0.4.0 → v1.0.0
**Focus:** High-level APIs, helpers, documentation polish

## Objectives

1. Create ergonomic high-level APIs
2. Add common use-case helpers
3. Polish documentation
4. Prepare for release

## Prerequisites

- Phases 1-3 completed
- Core functionality working
- Tests passing

## Convenience API Patterns

### Simple Wrappers
```rust
// src/conveniences/mod.rs

impl Client {
    /// Generate text with a simple prompt.
    ///
    /// This is a convenience wrapper around [`generate`](Self::generate).
    ///
    /// # Example
    ///
    /// ```no_run
    /// let text = client.prompt("llama3", "Hello!").await?;
    /// ```
    pub async fn prompt(&self, model: &str, prompt: &str) -> Result<String> {
        let request = GenerateRequest::new(model, prompt);
        let response = self.generate(request).await?;
        Ok(response.response)
    }
}
```

### Builder Extensions
```rust
// Extend request builders with common configurations

impl GenerateRequestBuilder {
    /// Configure for creative writing (high temperature).
    pub fn creative(mut self) -> Self {
        self.temperature(0.9)
            .top_p(0.95)
    }

    /// Configure for factual responses (low temperature).
    pub fn factual(mut self) -> Self {
        self.temperature(0.1)
            .top_p(0.5)
    }

    /// Configure for code generation.
    pub fn code(mut self) -> Self {
        self.temperature(0.2)
            .format(Format::Json)
    }
}
```

### Conversation Helpers
```rust
/// A conversation builder for chat interactions.
pub struct Conversation {
    client: Client,
    model: String,
    messages: Vec<ChatMessage>,
    options: ChatOptions,
}

impl Conversation {
    pub fn new(client: Client, model: impl Into<String>) -> Self {
        Self {
            client,
            model: model.into(),
            messages: Vec::new(),
            options: ChatOptions::default(),
        }
    }

    /// Add a system message.
    pub fn system(mut self, content: impl Into<String>) -> Self {
        self.messages.push(ChatMessage::system(content));
        self
    }

    /// Send a user message and get response.
    pub async fn send(&mut self, message: impl Into<String>) -> Result<String> {
        self.messages.push(ChatMessage::user(message));

        let request = ChatRequest::builder()
            .model(&self.model)
            .messages(self.messages.clone())
            .options(self.options.clone())
            .build()?;

        let response = self.client.chat(request).await?;

        self.messages.push(ChatMessage::assistant(&response.message.content));

        Ok(response.message.content)
    }
}
```

### Stream Collectors
```rust
/// Collect a stream into a complete response.
pub async fn collect_stream<S>(
    stream: S,
) -> Result<GenerateResponse>
where
    S: Stream<Item = Result<GenerateChunk>>,
{
    use tokio_stream::StreamExt;

    let mut full_response = String::new();
    let mut final_chunk = None;

    tokio::pin!(stream);

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        full_response.push_str(&chunk.response);
        final_chunk = Some(chunk);
    }

    let chunk = final_chunk.ok_or(LibraryError::EmptyStream)?;

    Ok(GenerateResponse {
        model: chunk.model,
        response: full_response,
        done: true,
        // ... other fields from final chunk
    })
}
```

## Feature Gating

```toml
# Cargo.toml

[features]
default = ["http"]
http = []
conveniences = ["http"]
```

```rust
// src/lib.rs

pub mod primitives;

#[cfg(feature = "http")]
pub mod http;

#[cfg(feature = "conveniences")]
pub mod conveniences;
```

## Documentation Polish

### README Updates
- Add all feature examples
- Update quick start
- Add troubleshooting section
- Add FAQ

### API Documentation
- Ensure all public items documented
- Add examples to complex functions
- Link related items with `[`]`
- Add module-level documentation

### Examples
- Create example for each major feature
- Add comments explaining code
- Test all examples compile and run

## Release Checklist

### Code Quality
- [ ] `cargo clippy -- -D warnings` passes
- [ ] `cargo fmt --check` passes
- [ ] All tests pass
- [ ] No compiler warnings

### Documentation
- [ ] All public items documented
- [ ] README is comprehensive
- [ ] CHANGELOG is up to date
- [ ] Examples are complete

### Package Metadata
- [ ] Cargo.toml metadata complete
- [ ] License file present
- [ ] Repository URL correct
- [ ] Documentation URL set

### Testing
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Examples compile
- [ ] Manual testing complete

### Version
- [ ] Version number updated
- [ ] CHANGELOG entry added
- [ ] Git tag created

## Convenience Features List

### Simple Operations
- [ ] `prompt(model, text)` - Quick text generation
- [ ] `chat_once(model, message)` - Single chat exchange
- [ ] `embed_text(model, text)` - Quick embedding

### Builders
- [ ] Creative mode preset
- [ ] Factual mode preset
- [ ] Code generation preset

### Conversations
- [ ] Conversation builder
- [ ] Message history management
- [ ] Context window helpers

### Streaming
- [ ] Stream collector
- [ ] Progress callback
- [ ] Cancellation support

### Utilities
- [ ] Model availability check
- [ ] Connection test
- [ ] Error retry helpers

## Testing Conveniences

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_prompt_convenience() {
        let client = MockClient::new();
        let text = client.prompt("model", "Hello").await.unwrap();
        assert!(!text.is_empty());
    }

    #[tokio::test]
    async fn test_conversation() {
        let client = MockClient::new();
        let mut conv = Conversation::new(client, "model")
            .system("You are helpful");

        let response = conv.send("Hello").await.unwrap();
        assert!(!response.is_empty());

        // History preserved
        assert_eq!(conv.messages.len(), 3); // system + user + assistant
    }
}
```

## Completion Criteria

1. Convenience APIs implemented
2. Feature-gated appropriately
3. All documentation complete
4. Examples for all features
5. Release checklist passed
6. Version 1.0.0 ready

## Post-Release

After v1.0.0:

1. Monitor for issues
2. Gather feedback
3. Plan v1.1.0 features
4. Maintain backward compatibility
