#[allow(warnings)]
mod bindings;

use cloud_ai::chat::{ChatChoice, ChatMessage, ChatRequest, ChatResponse};
use wstd::{
    http::{request::Request, Client, IntoBody},
    io::AsyncRead,
    runtime::block_on,
};

use crate::bindings::{exports::pawn::chat::handler::Guest, pawn::chat::types::Message};

impl Into<ChatMessage> for Message {
    fn into(self) -> ChatMessage {
        ChatMessage { role: self.role, content: self.content }
    }
}

impl From<ChatChoice> for Message {
    fn from(value: ChatChoice) -> Self {
        Message { role: value.message.role, content: value.message.content }
    }
}

struct Component;

impl Guest for Component {
    fn handle(provider: String, model: String, apikey: String, messages: Vec<Message>) -> Message {
        let endpoint = cloud_ai::get_chat_endpoint(&provider)
            .expect(&format!("{} is not supported", &provider));

        let messages =
            messages.iter().cloned().map(Into::<ChatMessage>::into).collect::<Vec<ChatMessage>>();
        let chat_request_body = ChatRequest::builder()
            .model(model)
            .messages(messages)
            .build()
            .expect("Fail to parse chat request");

        let body = serde_json::to_string(&chat_request_body).expect("Fail build json string");

        // Build the wstd request
        let req = Request::builder()
            .method("POST")
            .uri(endpoint)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", apikey))
            .body(body.into_body())
            .expect("Fail to build chat request");

        block_on(async {
            let client = Client::new();

            // Send request and read response
            let mut resp = client.send(req).await.expect("Request failed");
            let mut buf = vec![];
            let body = resp.body_mut();
            body.read_to_end(&mut buf).await.expect("Failed to read response body");
            let data = String::from_utf8(buf).expect("Invalid UTF-8 response");

            let chat_response =
                ChatResponse::from_json(&data).expect("Fail to parse chat response");
            Message::from(chat_response.choices[0].clone())
        })
    }
}

bindings::export!(Component with_types_in bindings);
