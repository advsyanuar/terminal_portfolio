pub mod buffer;
pub mod frontmatter;

use buffer::Buffer;
use crate::key::Key;
use crate::models::ContentType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorMode {
    Normal,
    Insert,
}

#[derive(Debug, Clone)]
pub struct Editor {
    pub buffer: Buffer,
    pub mode: EditorMode,
    pub slug: String,
    pub content_type: ContentType,
    pub title: String,
    pub description: String,
    pub date: String,
    pub tags: Vec<String>,
    pub has_frontmatter: bool,
    pub frontmatter_end: usize,
    pub dirty: bool,
    pub save_status: Option<String>,
}

impl Editor {
    pub fn new(content_type: ContentType, slug: String) -> Self {
        let template = format!(
            "# {}\n\nStart writing your {} content here...\n",
            slug,
            content_type.dir_name()
        );
        Self {
            buffer: Buffer::from_text(&format!(
                "---\ntitle: \ndate: \ndescription: \n---\n\n{}",
                template
            )),
            mode: EditorMode::Insert,
            slug,
            content_type,
            title: String::new(),
            description: String::new(),
            date: String::new(),
            tags: Vec::new(),
            has_frontmatter: true,
            frontmatter_end: 4,
            dirty: false,
            save_status: None,
        }
    }

    pub fn from_content(content_type: ContentType, slug: String, text: &str) -> Self {
        let lines: Vec<String> = text.lines().map(|l| l.to_string()).collect();
        let fm = frontmatter::extract_frontmatter(&lines);

        let (has_frontmatter, frontmatter_end) = if let Some(ref fm_block) = fm {
            (true, fm_block.end_line)
        } else {
            (false, 0)
        };

        Self {
            buffer: Buffer::from_text(text),
            mode: EditorMode::Insert,
            slug,
            content_type,
            title: String::new(),
            description: String::new(),
            date: String::new(),
            tags: Vec::new(),
            has_frontmatter,
            frontmatter_end,
            dirty: false,
            save_status: None,
        }
    }

    pub fn handle_key(&mut self, key: Key) -> bool {
        match key {
            Key::Up => {
                self.buffer.move_up();
                true
            }
            Key::Down => {
                self.buffer.move_down();
                true
            }
            Key::Left => {
                self.buffer.move_left();
                true
            }
            Key::Right => {
                self.buffer.move_right();
                true
            }
            Key::Char(c) => {
                self.buffer.insert_char(c);
                self.dirty = true;
                true
            }
            Key::Enter => {
                self.buffer.split_line();
                self.dirty = true;
                true
            }
            Key::Backspace => {
                self.buffer.delete_char();
                self.dirty = true;
                true
            }
            Key::Delete => {
                self.buffer.delete_forward();
                self.dirty = true;
                true
            }
            Key::Home => {
                self.buffer.move_home();
                true
            }
            Key::End => {
                self.buffer.move_end();
                true
            }
            Key::Tab => {
                self.buffer.insert_char(' ');
                self.buffer.insert_char(' ');
                self.dirty = true;
                true
            }
            _ => false,
        }
    }

    pub fn body_text(&self) -> String {
        if self.has_frontmatter {
            let fm_end = self.frontmatter_end;
            if fm_end + 1 < self.buffer.lines.len() {
                self.buffer.lines[fm_end + 1..].join("\n")
            } else {
                String::new()
            }
        } else {
            self.buffer.to_text()
        }
    }

    pub fn mark_saved(&mut self) {
        self.dirty = false;
        self.save_status = Some("Saved".to_string());
    }

    pub fn mark_error(&mut self, msg: &str) {
        self.save_status = Some(format!("Error: {}", msg));
    }
}
