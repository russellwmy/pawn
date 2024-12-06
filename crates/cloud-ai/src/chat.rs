use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder)]
#[builder(no_std)]
#[builder(setter(into))]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
}

impl ChatRequest {
    pub fn builder() -> ChatRequestBuilder {
        ChatRequestBuilder::create_empty()
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder)]
#[builder(no_std)]
#[builder(setter(into))]
pub struct ChatMessage {
    pub role: String, // "system", "user", or "assistant"
    pub content: String,
}

impl ChatMessage {
    pub fn builder() -> ChatMessageBuilder {
        ChatMessageBuilder::create_empty()
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ChatResponse {
    pub object: String,
    pub created: u64,
    pub choices: Vec<ChatChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<ChatUsage>,
}

impl ChatResponse {
    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ChatChoice {
    pub index: u32,
    pub message: ChatMessage,
    pub finish_reason: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ChatUsage {
    pub completion_tokens: u32,
    pub prompt_tokens: u32,
    pub total_tokens: u32,
}
