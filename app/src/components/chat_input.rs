use dioxus::prelude::*;

use crate::{backend::chat, components::Message};

#[component]
pub(crate) fn ChatInput(mut messages: Signal<Vec<Message>>) -> Element {
    let mut input_text = use_signal(String::new);
    let mut send_message = use_signal(|| false);

    rsx! {
        div { class: "flex p-2 gap-2 items-center",
            div { class: "flex-1",
                textarea {
                    class: "textarea textarea-accent w-full",
                    value: "{input_text}",
                    placeholder: "Type a message...",
                    oninput: move |e| input_text.set(e.value().clone()),
                    onkeydown: move |e| {
                        if e.key() == Key::Shift {
                            send_message.set(false);
                        }
                    },
                    onkeyup: move |e| async move {
                        if e.key() == Key::Shift {
                            send_message.set(true);
                        }
                        if e.key() == Key::Enter && *send_message.read()
                            && !input_text.to_string().is_empty()
                        {
                            let text = input_text.read().clone();
                            input_text.set("".to_string());
                            let message = Message {
                                role: "user".to_string(),
                                content: text,
                            };
                            messages.write().push(message);
                            let response = chat(messages()).await;
                            match response {
                                Ok(chatResponse) => {
                                    messages
                                        .write()
                                        .push(Message {
                                            role: chatResponse.role.to_string(),
                                            content: chatResponse.content.to_string(),
                                        });
                                }
                                _ => {}
                            };
                        }
                    },
                }
            }
        }
    }
}
