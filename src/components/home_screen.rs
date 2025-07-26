#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::Snippet;
use super::snippet_card::SnippetCard;

#[component]
pub fn HomeScreen(
    snippets: Vec<Snippet>,
    search_query: String,
    selected_language: String,
    on_search: EventHandler<String>,
    on_language_filter: EventHandler<String>,
    on_view_snippet: EventHandler<String>,
    on_edit_snippet: EventHandler<String>,
    on_delete_snippet: EventHandler<String>,
    on_toggle_favorite: EventHandler<String>,
) -> Element {
    let languages: Vec<String> = {
        let mut langs: std::collections::HashSet<String> = snippets.iter()
            .map(|s| s.language.clone())
            .collect();
        let mut lang_vec: Vec<String> = langs.into_iter().collect();
        lang_vec.sort();
        lang_vec
    };

    let filtered_snippets: Vec<Snippet> = snippets.into_iter()
        .filter(|snippet| {
            let matches_search = search_query.is_empty() || 
                snippet.title.to_lowercase().contains(&search_query.to_lowercase()) ||
                snippet.code.to_lowercase().contains(&search_query.to_lowercase()) ||
                snippet.tags.iter().any(|tag| tag.to_lowercase().contains(&search_query.to_lowercase()));
            
            let matches_language = selected_language.is_empty() || snippet.language == selected_language;
            
            matches_search && matches_language
        })
        .collect();

    rsx! {
        div { class: "home-screen",
            // Search and Filter Section
            div { class: "search-section",
                div { class: "search-bar",
                    input {
                        class: "search-input",
                        r#type: "text",
                        placeholder: "Search snippets...",
                        value: "{search_query}",
                        oninput: move |e| on_search.call(e.value())
                    }
                    span { class: "search-icon", "🔍" }
                }
                
                if !languages.is_empty() {
                    div { class: "filter-chips",
                        button {
                            class: if selected_language.is_empty() { "chip active" } else { "chip" },
                            onclick: move |_| on_language_filter.call(String::new()),
                            "All"
                        }
                        for language in languages {
                            button {
                                class: if selected_language == language { "chip active" } else { "chip" },
                                onclick: {
                                    let lang = language.clone();
                                    move |_| on_language_filter.call(lang.clone())
                                },
                                "{language}"
                            }
                        }
                    }
                }
            }

            // Snippets Grid
            if filtered_snippets.is_empty() {
                div { class: "empty-state",
                    div { class: "empty-icon", "📝" }
                    h3 { "No snippets found" }
                    p { 
                        if search_query.is_empty() && selected_language.is_empty() {
                            "Start by adding your first code snippet!"
                        } else {
                            "Try adjusting your search or filters"
                        }
                    }
                }
            } else {
                div { class: "snippets-grid",
                    for snippet in filtered_snippets {
                        SnippetCard {
                            key: "{snippet.id}",
                            snippet: snippet.clone(),
                            on_view: {
                                let id = snippet.id.clone();
                                move |_| on_view_snippet.call(id.clone())
                            },
                            on_edit: {
                                let id = snippet.id.clone();
                                move |_| on_edit_snippet.call(id.clone())
                            },
                            on_delete: {
                                let id = snippet.id.clone();
                                move |_| on_delete_snippet.call(id.clone())
                            },
                            on_toggle_favorite: {
                                let id = snippet.id.clone();
                                move |_| on_toggle_favorite.call(id.clone())
                            }
                        }
                    }
                }
            }
        }
    }
}
