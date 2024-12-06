use dioxus::prelude::*;

use super::chat_message::Message;
use crate::components::{ChatInput, ChatMessage};

#[component]
pub(crate) fn ChatView() -> Element {
    let messages = use_signal(Vec::<Message>::new);

    rsx!(
        div { class: "flex-1 flex flex-col",
            div { class: "flex-1 overflow-auto max-h-[calc(100vh-8rem)] gap-4",
                for msg in messages() {
                    ChatMessage { data: msg }
                }
            }
            div {
                ChatInput { messages }
            }
        }
    )
}
