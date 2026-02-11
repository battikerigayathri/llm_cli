// Placeholder for provider-specific implementations
// Will be expanded later for OpenAI, Anthropic, local models, etc.

pub trait LlmProvider {
    fn name(&self) -> &str;
    fn base_url(&self) -> &str;
}