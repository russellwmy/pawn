#[cfg(feature = "server")]
use std::fs;

use dioxus::{logger::tracing::info, prelude::*};
#[cfg(feature = "server")]
use pawn_runtime::{Component, Runtime};
#[cfg(feature = "server")]
use wasmtime::component::Val;

#[cfg(feature = "server")]
use crate::backend::config::get_config;
#[cfg(feature = "server")]
use crate::backend::config::Config;
use crate::components::Message;

#[server]
pub async fn chat(messages: Vec<Message>) -> Result<Message, ServerFnError> {
    let config: Config = get_config()?;
    let chat_config = config.chat;
    let wasm: Vec<u8> = fs::read(chat_config.component_wasm_path)?;
    let runtime: Runtime = Runtime::new()?;
    let component = Component::with_runtime(wasm.as_slice(), &runtime)?;
    let messages: Vec<Val> = messages
        .iter()
        .map(|m| {
            Val::Record(vec![
                ("role".to_string(), Val::String(m.role.to_string())),
                ("content".to_string(), Val::String(m.content.to_string())),
            ])
        })
        .collect();

    let results = component
        .call(Some(&chat_config.component_handler), &chat_config.component_handle_function, &vec![
            Val::String(chat_config.provider),
            Val::String(chat_config.model),
            Val::String(chat_config.api_key),
            Val::List(messages),
        ])
        .await?;

    let messages: Vec<Message> = results
        .iter()
        .filter_map(|r| {
            if let Val::Record(fields) = r {
                let role = fields
                    .iter()
                    .find(|(key, _)| key == "role") // Find field by name
                    .and_then(|(_, val)| match val {
                        Val::String(s) => Some(s.clone()), // Extract if it's a string
                        _ => None,
                    })?;

                let content = fields.iter().find(|(key, _)| key == "content").and_then(
                    |(_, val)| match val {
                        Val::String(s) => Some(s.clone()),
                        _ => None,
                    },
                )?;

                Some(Message { role, content })
            } else {
                None // Skip if it's not a Record
            }
        })
        .collect();

    Ok(messages[0].clone())
}
