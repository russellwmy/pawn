pub mod chat;

const GEMINI_CHAT_ENDPOINT: &str =
    "https://generativelanguage.googleapis.com/v1beta/chat/completions";
const OPENAI_CHAT_ENDPOINT: &str = "https://api.openai.com/v1/chat/completions";

pub fn get_chat_endpoint(provider: &str) -> Option<String> {
    match provider {
        "gemini" => Some(GEMINI_CHAT_ENDPOINT.to_string()),
        "openai" => Some(OPENAI_CHAT_ENDPOINT.to_string()),
        _ => None,
    }
}
