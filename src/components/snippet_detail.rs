#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::Snippet;

#[component]
pub fn SnippetDetail(
    snippet: Option<Snippet>,
    on_edit: EventHandler<String>,
    on_delete: EventHandler<String>,
    on_toggle_favorite: EventHandler<String>,
) -> Element {
    let mut ai_explanation = use_signal(|| None::<String>);
    let mut ai_loading = use_signal(|| false);

    if let Some(snippet) = snippet {
        // Clone the ID once outside the closures to avoid multiple moves
        let snippet_id = snippet.id.clone();
        let snippet_id_for_edit = snippet_id.clone();
        let snippet_id_for_delete = snippet_id.clone();
        let snippet_id_for_favorite = snippet_id.clone();
        
        rsx! {
            div { class: "snippet-detail",
                // Header section
                div { class: "detail-header",
                    div { class: "header-top",
                        div { class: "title-section",
                            h1 { class: "detail-title", "{snippet.title}" }
                            span { class: "language-badge {snippet.language}", "{snippet.language}" }
                        }
                        button {
                            class: if snippet.is_favorite { "btn btn-ghost favorite active" } else { "btn btn-ghost favorite" },
                            onclick: move |_| on_toggle_favorite.call(snippet_id_for_favorite.clone()),
                            "★"
                        }
                    }
                    if let Some(desc) = &snippet.description {
                        p { class: "detail-description", "{desc}" }
                    }
                    if !snippet.tags.is_empty() {
                        div { class: "tags-container",
                            for tag in &snippet.tags {
                                span { class: "tag", "#{tag}" }
                            }
                        }
                    }
                }

                // Code section
                div { class: "code-section",
                    div { class: "code-header",
                        div { class: "code-title", "Code" }
                        button {
                            class: "btn btn-sm btn-ghost",
                            onclick: move |_| {
                                // Simple clipboard copy - using console log for now
                                web_sys::console::log_1(&"Copy to clipboard clicked".into());
                                // TODO: implement proper clipboard copy when needed
                            },
                            "📋 Copy"
                        }
                    }
                    div { class: "code-container",
                        pre { class: "code-block {snippet.language}",
                            code { "{snippet.code}" }
                        }
                    }
                }

                // AI Explain section
                div { class: "ai-section",
                    button {
                        class: "btn btn-ai",
                        onclick: move |_| {
                            ai_loading.set(true);
                            let explanation = format!(
                                "This {} code snippet demonstrates {}. It's commonly used for {} and showcases {} programming concepts.",
                                snippet.language,
                                "core programming functionality",
                                "educational purposes and real-world applications",
                                "essential"
                            );
                            ai_explanation.set(Some(explanation));
                            ai_loading.set(false);
                        },
                        disabled: ai_loading(),
                        span { class: "btn-icon", "🤖" }
                        if ai_loading() { "Analyzing..." } else { "AI Explain" }
                    }

                    if let Some(expl) = ai_explanation() {
                        div { class: "ai-explanation",
                            div { class: "ai-header",
                                span { class: "ai-icon", "🤖" }
                                "AI Explanation"
                            }
                            p { class: "ai-text", "{expl}" }
                        }
                    }
                }

                // Action buttons
                div { class: "detail-actions",
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| on_edit.call(snippet_id_for_edit.clone()),
                        span { class: "btn-icon", "✏️" }
                        "Edit"
                    }
                    button {
                        class: "btn btn-danger",
                        onclick: move |_| {
                            // Simple confirmation using console for now
                            web_sys::console::log_1(&"Delete clicked".into());
                            on_delete.call(snippet_id_for_delete.clone());
                        },
                        span { class: "btn-icon", "🗑️" }
                        "Delete"
                    }
                    button {
                        class: "btn btn-secondary",
                        onclick: move |_| {
                            // Simple share - log to console for now
                            web_sys::console::log_1(&format!("Share: {}", snippet.title).into());
                        },
                        span { class: "btn-icon", "📤" }
                        "Share"
                    }
                }

                // Meta info
                div { class: "detail-meta",
                    p { "Created: {snippet.created_at}" }
                    p { "Updated: {snippet.updated_at}" }
                }
            }
        }
    } else {
        rsx! {
            div { class: "empty-state",
                div { class: "empty-icon", "❌" }
                h3 { "Snippet not found" }
                p { "The requested snippet could not be found." }
            }
        }
    }
}
