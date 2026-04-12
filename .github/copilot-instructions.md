# QAI SDK — GitHub Copilot Instructions

## Project Context
QAI SDK is a monolithic Rust crate (`qai-sdk`) providing a unified interface for 6+ AI providers. All providers implement shared traits (`LanguageModel`, `EmbeddingModel`, etc.) from `src/core/`.

## Key Rules
1. **Error Handling**: Return `Result<T, ProviderError>`. Never `unwrap()` in library code.
2. **Feature Gates**: All provider modules are behind `#[cfg(feature = "...")]` flags.
3. **Async**: All trait methods use `#[async_trait]` and run on tokio.
4. **Types**: Provider-specific request/response types are `pub(crate)`.
5. **Imports**: Consumer code uses `use qai_sdk::prelude::*;`.

## File Layout
- `src/core/` — Shared traits, types, error, structured output, registry, middleware, agent
- `src/<provider>/` — Provider implementations (openai, anthropic, google, deepseek, xai, openai_compatible)
- `src/mcp/` — Model Context Protocol client and agent
- `examples/` — Runnable examples
- `docs/` — Module documentation

## Code Patterns
```rust
// Provider factory
pub fn create_openai(settings: ProviderSettings) -> OpenAIModel { ... }

// Trait implementation
#[async_trait]
impl LanguageModel for OpenAIModel {
    async fn generate(&self, prompt: Prompt, options: GenerateOptions) -> Result<GenerateResult, ProviderError> { ... }
    async fn generate_stream(&self, prompt: Prompt, options: GenerateOptions) -> Result<BoxStream<'static, StreamPart>, ProviderError> { ... }
}

// Error conversion
impl From<reqwest::Error> for ProviderError {
    fn from(err: reqwest::Error) -> Self { ProviderError::Network(err.to_string()) }
}
```

## Testing
- Use `#[tokio::test]` for async tests
- Mock with `MockLanguageModel` from `src/test_utils/`
- Never call real APIs in unit tests
