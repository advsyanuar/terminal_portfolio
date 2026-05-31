use crate::models::{ContentData, ContentType, EntryInfo};

#[cfg(not(target_arch = "wasm32"))]
pub mod local;
#[cfg(target_arch = "wasm32")]
pub mod remote;

pub trait Storage {
    fn list_entries(&mut self, content_type: &ContentType) -> Result<Vec<EntryInfo>, String>;
    fn load_entry(&mut self, content_type: &ContentType, slug: &str)
        -> Result<ContentData, String>;
    fn save_entry(
        &mut self,
        content_type: &ContentType,
        slug: &str,
        frontmatter: &serde_json::Value,
        body: &str,
    ) -> Result<(), String>;
    fn delete_entry(&mut self, content_type: &ContentType, slug: &str) -> Result<(), String>;
}
