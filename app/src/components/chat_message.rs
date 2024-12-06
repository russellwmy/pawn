use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::Avatar;

#[derive(Debug, PartialEq, Props, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[component]
pub(crate) fn ChatMessage(data: Message) -> Element {
    let mut html_output = String::new();

    let parser = pulldown_cmark::Parser::new(&data.content);
    pulldown_cmark::html::push_html(&mut html_output, parser);

    use_effect(move || {
        document::eval("hljs.highlightAll();");
    });

    rsx! {
        if &data.role == "user" {
            div { class: "chat chat-end",
                div { class: "chat-bubble", dangerous_inner_html: html_output }
            }
        } else {
            div { class: "chat chat-start max-w-3xl",
                div {
                    class: "chat-bubble markdown",
                    dangerous_inner_html: html_output,
                }
            }
        }
    }
}
