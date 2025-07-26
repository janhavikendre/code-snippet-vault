#![allow(non_snake_case)]
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

mod components;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Snippet {
    pub id: String,
    pub title: String,
    pub language: String,
    pub code: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
    pub is_favorite: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Screen {
    Home,
    Add,
    Edit(String),
    View(String),
}

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let mut snippets = use_signal(|| get_sample_snippets());
    let mut current_screen = use_signal(|| Screen::Home);
    let mut search_query = use_signal(|| String::new());
    let mut selected_language = use_signal(|| String::new());

    rsx! {
        style { {include_str!("../assets/styles.css")} }
        div { class: "app-container",
            // Header
            header { class: "app-header",
                div { class: "header-content",
                    div { class: "header-left",
                        h1 { class: "app-title",
                            span { class: "app-icon", "📱" }
                            "Code Vault"
                        }
                    }
                    div { class: "header-right",
                        if matches!(current_screen(), Screen::Home) {
                            button { 
                                class: "btn btn-primary btn-sm",
                                onclick: move |_| current_screen.set(Screen::Add),
                                span { class: "btn-icon", "+" }
                                "Add"
                            }
                        }
                        if !matches!(current_screen(), Screen::Home) {
                            button { 
                                class: "btn btn-ghost btn-sm",
                                onclick: move |_| current_screen.set(Screen::Home),
                                span { class: "btn-icon", "←" }
                                "Back"
                            }
                        }
                    }
                }
            }

            // Main Content
            main { class: "main-content",
                match current_screen() {
                    Screen::Home => rsx! {
                        components::home_screen::HomeScreen {
                            snippets: snippets(),
                            search_query: search_query(),
                            selected_language: selected_language(),
                            on_search: move |query: String| search_query.set(query),
                            on_language_filter: move |lang: String| selected_language.set(lang),
                            on_view_snippet: move |id: String| current_screen.set(Screen::View(id)),
                            on_edit_snippet: move |id: String| current_screen.set(Screen::Edit(id)),
                            on_delete_snippet: move |id: String| {
                                snippets.write().retain(|s| s.id != id);
                            },
                            on_toggle_favorite: move |id: String| {
                                if let Some(snippet) = snippets.write().iter_mut().find(|s| s.id == id) {
                                    snippet.is_favorite = !snippet.is_favorite;
                                }
                            }
                        }
                    },
                    Screen::Add => rsx! {
                        components::add_edit_snippet::AddEditScreen {
                            snippet: None,
                            on_save: move |snippet: Snippet| {
                                snippets.write().push(snippet);
                                current_screen.set(Screen::Home);
                            },
                            on_cancel: move |_| current_screen.set(Screen::Home)
                        }
                    },
                    Screen::Edit(id) => {
                        let snippet = snippets().iter().find(|s| s.id == id).cloned();
                        rsx! {
                            components::add_edit_snippet::AddEditScreen {
                                snippet,
                                on_save: move |updated_snippet: Snippet| {
                                    let snippet_id = updated_snippet.id.clone();
                                    if let Some(snippet) = snippets.write().iter_mut().find(|s| s.id == updated_snippet.id) {
                                        *snippet = updated_snippet;
                                    }
                                    current_screen.set(Screen::View(snippet_id));
                                },
                                on_cancel: move |_| current_screen.set(Screen::Home)
                            }
                        }
                    },
                    Screen::View(id) => {
                        let snippet = snippets().iter().find(|s| s.id == id).cloned();
                        rsx! {
                            components::snippet_detail::SnippetDetail {
                                snippet,
                                on_edit: move |id: String| current_screen.set(Screen::Edit(id)),
                                on_delete: move |id: String| {
                                    snippets.write().retain(|s| s.id != id);
                                    current_screen.set(Screen::Home);
                                },
                                on_toggle_favorite: move |id: String| {
                                    if let Some(snippet) = snippets.write().iter_mut().find(|s| s.id == id) {
                                        snippet.is_favorite = !snippet.is_favorite;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Bottom Navigation (Mobile)
            nav { class: "bottom-nav",
                button {
                    class: if matches!(current_screen(), Screen::Home) { "nav-btn active" } else { "nav-btn" },
                    onclick: move |_| current_screen.set(Screen::Home),
                    span { class: "nav-icon", "🏠" }
                    span { class: "nav-label", "Home" }
                }
                button {
                    class: "nav-btn",
                    onclick: move |_| {
                        // Future search functionality
                        web_sys::console::log_1(&"Search clicked".into());
                    },
                    span { class: "nav-icon", "🔍" }
                    span { class: "nav-label", "Search" }
                }
                button {
                    class: if matches!(current_screen(), Screen::Add) { "nav-btn active" } else { "nav-btn" },
                    onclick: move |_| current_screen.set(Screen::Add),
                    span { class: "nav-icon", "+" }
                    span { class: "nav-label", "Add" }
                }
            }
        }
    }
}

fn get_sample_snippets() -> Vec<Snippet> {
    vec![
        Snippet {
            id: "1".into(),
            title: "Hello World in Rust".into(),
            language: "rust".into(),
            code: "fn main() {\n    println!(\"Hello, world!\");\n}".into(),
            description: Some("A simple Hello World program in Rust".into()),
            tags: vec!["beginner".into(), "rust".into(), "hello-world".into()],
            created_at: "2024-01-15".into(),
            updated_at: "2024-01-15".into(),
            is_favorite: true,
        },
        Snippet {
            id: "2".into(),
            title: "JavaScript For Loop".into(),
            language: "javascript".into(),
            code: "for (let i = 0; i < 10; i++) {\n    console.log(`Count: ${i}`);\n}".into(),
            description: Some("Basic for loop example in JavaScript".into()),
            tags: vec!["javascript".into(), "loop".into(), "basics".into()],
            created_at: "2024-01-14".into(),
            updated_at: "2024-01-14".into(),
            is_favorite: false,
        },
        Snippet {
            id: "3".into(),
            title: "Python List Comprehension".into(),
            language: "python".into(),
            code: "squares = [x**2 for x in range(10)]\nprint(squares)".into(),
            description: Some("Creating a list of squares using list comprehension".into()),
            tags: vec!["python".into(), "list-comprehension".into(), "functional".into()],
            created_at: "2024-01-13".into(),
            updated_at: "2024-01-13".into(),
            is_favorite: true,
        },
    ]
}

