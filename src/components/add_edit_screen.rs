use dioxus::prelude::*;
use crate::Snippet;

#[component]
pub fn AddEditScreen(
    snippet: Option<Snippet>,
    on_save: EventHandler<Snippet>,
    on_cancel: EventHandler<()>,
) -> Element {
    let is_editing = snippet.is_some();
    let snippet = snippet.unwrap_or_else(|| Snippet {
        id: format!("{}", js_sys::Date::now() as u64),
        title: String::new(),
        language: "rust".into(),
        code: String::new(),
        description: None,
        tags: vec![],
        created_at: "2024-01-15".into(),
        updated_at: "2024-01-15".into(),
        is_favorite: false,
    });

    let mut title = use_signal(|| snippet.title.clone());
    let mut language = use_signal(|| snippet.language.clone());
    let mut code = use_signal(|| snippet.code.clone());
    let mut description = use_signal(|| snippet.description.unwrap_or_default());
    let mut tags_input = use_signal(|| snippet.tags.join(", "));

    let languages = vec![
        "rust", "javascript", "python", "typescript", "go", "java", 
        "cpp", "c", "swift", "kotlin", "dart", "php", "ruby", "html", "css"
    ];

    rsx! {
        div { class: "add-edit-screen",
            div { class: "form-container",
                h2 { class: "form-title",
                    if is_editing { "Edit Snippet" } else { "Add New Snippet" }
                }

                form { 
                    class: "snippet-form",
                    onsubmit: move |e| {
                        e.prevent_default();
                        let tags: Vec<String> = tags_input.read()
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();

                        let new_snippet = Snippet {
                            id: snippet.id.clone(),
                            title: title.read().clone(),
                            language: language.read().clone(),
                            code: code.read().clone(),
                            description: if description.read().is_empty() { None } else { Some(description.read().clone()) },
                            tags,
                            created_at: snippet.created_at.clone(),
                            updated_at: "2024-01-15".into(),
                            is_favorite: snippet.is_favorite,
                        };
                        on_save.call(new_snippet);
                    },

                    div { class: "form-group",
                        label { class: "form-label", "Title" }
                        input {
                            class: "form-input",
                            r#type: "text",
                            placeholder: "Enter snippet title...",
                            value: "{title}",
                            oninput: move |e| title.set(e.value()),
                            required: true
                        }
                    }

                    div { class: "form-group",
                        label { class: "form-label", "Language" }
                        select {
                            class: "form-select",
                            value: "{language}",
                            onchange: move |e| language.set(e.value()),
                            for lang in languages {
                                option { 
                                    value: "{lang}",
                                    selected: *language.read() == lang,
                                    "{lang}"
                                }
                            }
                        }
                    }

                    div { class: "form-group",
                        label { class: "form-label", "Description (Optional)" }
                        textarea {
                            class: "form-textarea",
                            placeholder: "Brief description of what this snippet does...",
                            value: "{description}",
                            oninput: move |e| description.set(e.value()),
                            rows: "2"
                        }
                    }

                    div { class: "form-group",
                        label { class: "form-label", "Code" }
                        textarea {
                            class: "form-textarea code-input",
                            placeholder: "Paste your code here...",
                            value: "{code}",
                            oninput: move |e| code.set(e.value()),
                            rows: "12",
                            required: true
                        }
                    }

                    div { class: "form-group",
                        label { class: "form-label", "Tags (comma separated)" }
                        input {
                            class: "form-input",
                            r#type: "text",
                            placeholder: "e.g., beginner, algorithm, api",
                            value: "{tags_input}",
                            oninput: move |e| tags_input.set(e.value())
                        }
                    }

                    div { class: "form-actions",
                        button {
                            class: "btn btn-primary",
                            r#type: "submit",
                            span { class: "btn-icon", "💾" }
                            if is_editing { "Update Snippet" } else { "Save Snippet" }
                        }
                        button {
                            class: "btn btn-secondary",
                            r#type: "button",
                            onclick: move |_| on_cancel.call(()),
                            "Cancel"
                        }
                    }
                }
            }
        }
    }
}
