#![allow(non_snake_case)]
use dioxus::prelude::*;

/// Component to display AI explanation with loading indicator.
#[component]
pub fn AIExplain(
    explanation: Option<String>,
    loading: bool,
    on_request: EventHandler<()>
) -> Element {
    rsx! {
        div { class: "ai-section",
            button {
                class: "btn btn-ai",
                onclick: move |_| on_request.call(()),
                disabled: loading,
                span { class: "btn-icon", "🤖" }
                if loading { "Analyzing..." } else { "AI Explain" }
            }
            if let Some(expl) = explanation {
                div { class: "ai-explanation",
                    div { class: "ai-header",
                        span { class: "ai-icon", "🤖" }
                        "AI Explanation"
                    }
                    p { class: "ai-text", "{expl}" }
                }
            }
        }
    }
}

/// A simple version used in SnippetCard.
#[component]
pub fn AiExplainButton() -> Element {
    rsx! {
        button {
            class: "mt-2 bg-purple-600 hover:bg-purple-800 px-3 py-1 text-sm rounded",
            onclick: move |_| {
                println!("🤖  AI explain clicked – integrate GPT here");
            },
            "🤖 Explain with AI"
        }
    }
}

