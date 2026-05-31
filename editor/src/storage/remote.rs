use crate::models::{ContentData, ContentType, EntryInfo, SaveRequest};
use crate::storage::Storage;

pub struct RemoteStorage {
    base_url: String,
    client: reqwest::Client,
}

impl RemoteStorage {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client: reqwest::Client::new(),
        }
    }
}

impl Storage for RemoteStorage {
    fn list_entries(&mut self, _content_type: &ContentType) -> Result<Vec<EntryInfo>, String> {
        Err("Browsing existing entries not available in WASM mode".to_string())
    }

    fn load_entry(
        &mut self,
        _content_type: &ContentType,
        _slug: &str,
    ) -> Result<ContentData, String> {
        Err("Loading existing entries not available in WASM mode".to_string())
    }

    fn save_entry(
        &mut self,
        content_type: &ContentType,
        slug: &str,
        frontmatter: &serde_json::Value,
        body: &str,
    ) -> Result<(), String> {
        let url = format!("{}/api/save-content", self.base_url);
        let req = SaveRequest {
            content_type: content_type.dir_name().to_string(),
            slug: slug.to_string(),
            frontmatter: frontmatter.clone(),
            body: body.to_string(),
        };

        let client = self.client.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let _ = client.post(&url).json(&req).send().await;
        });

        Ok(())
    }

    fn delete_entry(&mut self, _content_type: &ContentType, _slug: &str) -> Result<(), String> {
        Err("Delete not available in WASM mode".to_string())
    }
}
