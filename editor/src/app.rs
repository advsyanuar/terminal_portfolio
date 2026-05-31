use crate::editor::Editor;
use crate::key::Key;
use crate::models::{ContentType, EntryInfo};
use crate::storage::Storage;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Dashboard,
    Browser,
    Editor,
    Prompt,
}

pub struct PromptState {
    pub label: String,
    pub value: String,
    pub cursor: usize,
    pub error: Option<String>,
}

impl PromptState {
    fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            value: String::new(),
            cursor: 0,
            error: None,
        }
    }
}

pub struct App {
    pub screen: Screen,
    pub content_type: ContentType,
    pub entries: Vec<EntryInfo>,
    pub blog_entries: Vec<EntryInfo>,
    pub project_entries: Vec<EntryInfo>,
    pub browser_cursor: usize,
    pub editor: Option<Editor>,
    pub prompt: Option<PromptState>,
    pub status_message: Option<String>,
    pub pending_slug: Option<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            screen: Screen::Dashboard,
            content_type: ContentType::Blog,
            entries: Vec::new(),
            blog_entries: Vec::new(),
            project_entries: Vec::new(),
            browser_cursor: 0,
            editor: None,
            prompt: None,
            status_message: None,
            pending_slug: None,
        }
    }

    pub fn handle_key(&mut self, key: Key, storage: &mut dyn Storage) {
        match self.screen {
            Screen::Dashboard => self.handle_dashboard_key(key, storage),
            Screen::Browser => self.handle_browser_key(key, storage),
            Screen::Editor => self.handle_editor_key(key, storage),
            Screen::Prompt => self.handle_prompt_key(key, storage),
        }
    }

    fn handle_dashboard_key(&mut self, key: Key, storage: &mut dyn Storage) {
        match key {
            Key::Up | Key::Down => {
                self.content_type = match self.content_type {
                    ContentType::Blog => ContentType::Project,
                    ContentType::Project => ContentType::Blog,
                };
            }
            Key::Enter => {
                self.browser_cursor = 0;
                self.entries = match storage.list_entries(&self.content_type) {
                    Ok(entries) => entries,
                    Err(e) => {
                        self.status_message = Some(format!("Failed to list: {}", e));
                        Vec::new()
                    }
                };
                self.screen = Screen::Browser;
            }
            _ => {}
        }
    }

    fn handle_browser_key(&mut self, key: Key, storage: &mut dyn Storage) {
        match key {
            Key::Up => {
                if self.browser_cursor > 0 {
                    self.browser_cursor -= 1;
                }
            }
            Key::Down => {
                let max = self.entries.len();
                if self.browser_cursor < max {
                    self.browser_cursor += 1;
                }
            }
            Key::Char('n') | Key::Char('N') => {
                self.screen = Screen::Prompt;
                self.prompt = Some(PromptState::new("Enter slug:"));
            }
            Key::Enter => {
                if self.browser_cursor < self.entries.len() {
                    let entry = &self.entries[self.browser_cursor];
                    match storage.load_entry(&self.content_type, &entry.slug) {
                        Ok(data) => {
                            let body = data.body;
                            let frontmatter = data.frontmatter;
                            let text = format!("---\n{}---\n\n{}", frontmatter_to_text(&frontmatter), body);
                            self.editor = Some(Editor::from_content(
                                self.content_type.clone(),
                                entry.slug.clone(),
                                &text,
                            ));
                            self.screen = Screen::Editor;
                        }
                        Err(e) => {
                            self.status_message = Some(format!("Failed to load: {}", e));
                        }
                    }
                }
            }
            Key::Esc => {
                self.screen = Screen::Dashboard;
            }
            _ => {}
        }
    }

    fn handle_editor_key(&mut self, key: Key, storage: &mut dyn Storage) {
        match key {
            Key::Esc => {
                if let Some(ref editor) = self.editor {
                    if editor.dirty {
                        self.screen = Screen::Prompt;
                        self.prompt = Some(PromptState {
                            label: "Unsaved changes. Save? (y/n):".to_string(),
                            value: String::new(),
                            cursor: 0,
                            error: None,
                        });
                        return;
                    }
                }
                self.editor = None;
                self.screen = Screen::Browser;
            }
            Key::CtrlS => {
                self.save_current(storage);
            }
            _ => {
                if let Some(ref mut editor) = self.editor {
                    editor.handle_key(key);
                }
            }
        }
    }

    fn handle_prompt_key(&mut self, key: Key, storage: &mut dyn Storage) {
        match key {
            Key::Char(c) => {
                if let Some(ref mut p) = self.prompt {
                    p.value.insert(p.cursor, c);
                    p.cursor += 1;
                }
            }
            Key::Backspace => {
                if let Some(ref mut p) = self.prompt {
                    if p.cursor > 0 {
                        p.value.remove(p.cursor - 1);
                        p.cursor -= 1;
                    }
                }
            }
            Key::Enter => {
                let value = self.prompt.as_ref().map(|p| p.value.clone()).unwrap_or_default();
                let label = self.prompt.as_ref().map(|p| p.label.clone()).unwrap_or_default();

                if label.starts_with("Unsaved") {
                    if value.to_lowercase() == "y" {
                        self.save_current(storage);
                    }
                    self.editor = None;
                    self.screen = Screen::Browser;
                    self.prompt = None;
                    return;
                }

                self.prompt = None;
                self.create_new_entry(&value);
            }
            Key::Esc => {
                self.prompt = None;
                self.screen = Screen::Browser;
            }
            _ => {}
        }
    }

    fn save_current(&mut self, storage: &mut dyn Storage) {
        if let Some(ref mut editor) = self.editor {
            let body = editor.body_text();
            let fm = if editor.content_type == ContentType::Blog {
                let post = crate::models::BlogPost {
                    title: editor.title.clone(),
                    date: editor.date.clone(),
                    description: editor.description.clone(),
                    tags: editor.tags.clone(),
                    draft: false,
                    image: None,
                };
                crate::editor::frontmatter::build_blog_frontmatter(&post)
            } else {
                let project = crate::models::Project {
                    title: editor.title.clone(),
                    description: editor.description.clone(),
                    date: editor.date.clone(),
                    tech_stack: editor.tags.clone(),
                    links: crate::models::ProjectLinks::default(),
                    featured: false,
                };
                crate::editor::frontmatter::build_project_frontmatter(&project)
            };

            match storage.save_entry(&editor.content_type, &editor.slug, &fm, &body) {
                Ok(()) => {
                    editor.mark_saved();
                    self.status_message = Some("Saved successfully".to_string());
                }
                Err(e) => {
                    editor.mark_error(&e);
                    self.status_message = Some(format!("Save failed: {}", e));
                }
            }
        }
    }

    pub fn load_entries(&mut self, json: &str) {
        if let Ok(data) = serde_json::from_str::<serde_json::Value>(json) {
            if let Some(blog) = data.get("blog").and_then(|v| v.as_array()) {
                self.blog_entries = blog
                    .iter()
                    .filter_map(|e| {
                        Some(EntryInfo {
                            slug: e.get("slug")?.as_str()?.to_string(),
                            title: e.get("title")?.as_str()?.to_string(),
                            date: e.get("date").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                            description: e.get("description").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                            content_type: ContentType::Blog,
                        })
                    })
                    .collect();
            }
            if let Some(proj) = data.get("projects").and_then(|v| v.as_array()) {
                self.project_entries = proj
                    .iter()
                    .filter_map(|e| {
                        Some(EntryInfo {
                            slug: e.get("slug")?.as_str()?.to_string(),
                            title: e.get("title")?.as_str()?.to_string(),
                            date: e.get("date").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                            description: e.get("description").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                            content_type: ContentType::Project,
                        })
                    })
                    .collect();
            }
        }
    }

    fn create_new_entry(&mut self, slug: &str) {
        let slug = slug.trim().to_lowercase().replace(' ', "-");
        if slug.is_empty() {
            self.status_message = Some("Slug cannot be empty".to_string());
            return;
        }
        self.editor = Some(Editor::new(self.content_type.clone(), slug));
        self.screen = Screen::Editor;
    }
}

fn frontmatter_to_text(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Object(map) => {
            let mut out = String::new();
            for (key, val) in map {
                let line = match val {
                    serde_json::Value::String(s) => format!("{}: {}", key, s),
                    serde_json::Value::Bool(b) => format!("{}: {}", key, b),
                    serde_json::Value::Number(n) => format!("{}: {}", key, n),
                    serde_json::Value::Array(arr) => {
                        let items: Vec<String> = arr
                            .iter()
                            .map(|v| match v {
                                serde_json::Value::String(s) => s.clone(),
                                _ => format!("{}", v),
                            })
                            .collect();
                        format!("{}: [{}]", key, items.join(", "))
                    }
                    _ => continue,
                };
                out.push_str(&line);
                out.push('\n');
            }
            out
        }
        _ => String::new(),
    }
}
