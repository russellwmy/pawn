use dioxus::prelude::*;

use super::chat_message::Message;
use crate::components::{ChatInput, ChatMessage};

#[component]
pub(crate) fn Avatar() -> Element {
    rsx!(
        div { class: "avatar",
            div { class: "ring-primary ring-offset-base-100 w-12 rounded-full ring ring-offset-2",
                img { src: "https://img.daisyui.com/images/stock/photo-1534528741775-53994a69daeb.webp" }
            }
        }
    )
}
