use dioxus::prelude::*;

mod backend;

mod components;

use components::ChatView;

const APP_CSS: Asset = asset!("/assets/app.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: APP_CSS }
        document::Link {
            rel: "stylesheet",
            href: "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/styles/dark.min.css",
        }
        document::Script { src: "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/highlight.min.js" }
        document::Script { src: "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/languages/rust.min.js" }
        document::Script { src: "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/languages/python.min.js" }
        div { class: "w-screen h-screen",
            div { class: "h-full w-full flex flex-col p-2", ChatView {} }
        }
    }
}
