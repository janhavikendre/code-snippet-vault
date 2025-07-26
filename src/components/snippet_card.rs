#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::Snippet;

#[component]
pub fn SnippetCard(
    snippet: Snippet,
    on_view: EventHandler<()>,
    on_edit: EventHandler<()>,
    on_delete: EventHandler<()>,
    on_toggle_favorite: EventHandler<()>,
) -> Element {
    let code_preview = if snippet.code.len() > 100 {
        format!("{}...", &snippet.code[..100])
    } else {
        snippet.code.clone()
    };

    rsx! {
        div { 
            class: "snippet-card",
            onclick: move |_| on_view.call(()),
            
            div { class: "card-header",
                div { class: "card-title-section",
                    h3 { class: "card-title", "{snippet.title}" }
                    span { class: "language-badge {snippet.language}", "{snippet.language}" }
                }
                
                div { class: "card-actions",
                    button {
                        class: if snippet.is_favorite { "action-btn favorite active" } else { "action-btn favorite" },
                        onclick: move |e| {
                            e.stop_propagation();
                            on_toggle_favorite.call(());
                        },
                        "★"
                    }
                    button {
                        class: "action-btn",
                        onclick: move |e| {
                            e.stop_propagation();
                            on_edit.call(());
                        },
                        "✏️"
                    }
                    button {
                        class: "action-btn delete",
                        onclick: move |e| {
                            e.stop_propagation();
                            on_delete.call(());
                        },
                        "🗑️"
                    }
                }
            }

            if let Some(description) = &snippet.description {
                p { class: "card-description", "{description}" }
            }

            div { class: "code-preview",
                pre { class: "code-block {snippet.language}",
                    code { "{code_preview}" }
                }
            }

            if !snippet.tags.is_empty() {
                div { class: "tags-container",
                    for tag in &snippet.tags {
                        span { class: "tag", "#{tag}" }
                    }
                }
            }

            div { class: "card-footer",
                span { class: "date", "Updated {snippet.updated_at}" }
            }
        }
    }
}
