use std::path::PathBuf;

use crate::editor::frontmatter;
use crate::models::{ContentData, ContentType, EntryInfo};
use crate::storage::Storage;

pub struct LocalStorage {
    base_path: PathBuf,
}

impl LocalStorage {
    pub fn new(base_path: &str) -> Self {
        Self {
            base_path: PathBuf::from(base_path),
        }
    }

    fn content_dir(&self, content_type: &ContentType) -> PathBuf {
        self.base_path.join("src").join("content").join(content_type.dir_name())
    }
}

impl Storage for LocalStorage {
    fn list_entries(&mut self, content_type: &ContentType) -> Result<Vec<EntryInfo>, String> {
        let dir = self.content_dir(content_type);
        if !dir.exists() {
            return Ok(Vec::new());
        }
        let mut entries = Vec::new();
        let rd = std::fs::read_dir(&dir).map_err(|e| e.to_string())?;
        for entry in rd {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            if path.extension().map(|e| e == "md" || e == "mdx").unwrap_or(false) {
                let slug = path
                    .file_stem()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_default();
                let (title, date, description) = parse_entry_metadata(&path);
                entries.push(EntryInfo {
                    slug,
                    title,
                    date,
                    description,
                    content_type: content_type.clone(),
                });
            }
        }
        entries.sort_by(|a, b| a.slug.cmp(&b.slug));
        Ok(entries)
    }

    fn load_entry(
        &mut self,
        content_type: &ContentType,
        slug: &str,
    ) -> Result<ContentData, String> {
        let dir = self.content_dir(content_type);
        let path = dir.join(format!("{}.md", slug));
        let text = std::fs::read_to_string(&path).map_err(|e| format!("failed to read {}: {}", path.display(), e))?;

        let lines: Vec<String> = text.lines().map(|l| l.to_string()).collect();
        let fm = frontmatter::extract_frontmatter(&lines);

        let (frontmatter, body) = if let Some(fm_block) = fm {
            let fm_value = parse_frontmatter_raw(&fm_block.raw);
            let body_start = fm_block.end_line + 1;
            let body = if body_start < lines.len() {
                lines[body_start..].join("\n")
            } else {
                String::new()
            };
            (fm_value, body)
        } else {
            (serde_json::Value::Object(serde_json::Map::new()), text)
        };

        Ok(ContentData { frontmatter, body })
    }

    fn save_entry(
        &mut self,
        content_type: &ContentType,
        slug: &str,
        frontmatter: &serde_json::Value,
        body: &str,
    ) -> Result<(), String> {
        let dir = self.content_dir(content_type);
        std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

        let fm_text = frontmatter::serialize_frontmatter(frontmatter)?;
        let full_text = format!("---\n{}---\n\n{}", fm_text, body);

        let path = dir.join(format!("{}.md", slug));
        std::fs::write(&path, &full_text).map_err(|e| format!("failed to write {}: {}", path.display(), e))?;
        Ok(())
    }

    fn delete_entry(&mut self, content_type: &ContentType, slug: &str) -> Result<(), String> {
        let dir = self.content_dir(content_type);
        let md_path = dir.join(format!("{}.md", slug));
        let mdx_path = dir.join(format!("{}.mdx", slug));
        if md_path.exists() {
            std::fs::remove_file(&md_path).map_err(|e| e.to_string())?;
        } else if mdx_path.exists() {
            std::fs::remove_file(&mdx_path).map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}

fn parse_entry_metadata(path: &std::path::Path) -> (String, String, String) {
    let text = std::fs::read_to_string(path).unwrap_or_default();
    let lines: Vec<&str> = text.lines().collect();
    if lines.len() >= 3 && lines[0].trim() == "---" {
        let end = lines[1..].iter().position(|l| l.trim() == "---").map(|i| i + 1).unwrap_or(0);
        let fm_lines = &lines[1..end];
        let mut title = String::new();
        let mut date = String::new();
        let mut desc = String::new();
        for line in fm_lines {
            if let Some((k, v)) = line.split_once(':') {
                let k = k.trim();
                let v = v.trim().to_string();
                match k {
                    "title" => title = v,
                    "date" => date = v,
                    "description" => desc = v,
                    _ => {}
                }
            }
        }
        (title, date, desc)
    } else {
        (String::new(), String::new(), String::new())
    }
}

fn parse_frontmatter_raw(raw: &str) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    for line in raw.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some((key, val)) = line.split_once(':') {
            let key = key.trim().to_string();
            let val = val.trim().to_string();
            map.insert(key, serde_json::Value::String(val));
        }
    }
    serde_json::Value::Object(map)
}
