use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogPost {
    pub title: String,
    pub date: String,
    pub description: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub draft: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub date: String,
    #[serde(default)]
    pub tech_stack: Vec<String>,
    #[serde(default)]
    pub links: ProjectLinks,
    #[serde(default)]
    pub featured: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectLinks {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demo: Option<String>,
}

impl Default for ProjectLinks {
    fn default() -> Self {
        Self {
            source: None,
            demo: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentType {
    Blog,
    Project,
}

impl ContentType {
    pub fn dir_name(&self) -> &str {
        match self {
            ContentType::Blog => "blog",
            ContentType::Project => "projects",
        }
    }

    pub fn label(&self) -> &str {
        match self {
            ContentType::Blog => "BLOG",
            ContentType::Project => "PROJECTS",
        }
    }
}

pub enum ContentEntry {
    Blog(BlogPost),
    Project(Project),
}

impl ContentEntry {
    pub fn title(&self) -> &str {
        match self {
            ContentEntry::Blog(p) => &p.title,
            ContentEntry::Project(p) => &p.title,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EntryInfo {
    pub slug: String,
    pub title: String,
    pub date: String,
    pub description: String,
    pub content_type: ContentType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveRequest {
    #[serde(rename = "type")]
    pub content_type: String,
    pub slug: String,
    pub frontmatter: serde_json::Value,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRequest {
    #[serde(rename = "type")]
    pub content_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListEntry {
    pub name: String,
    pub slug: String,
    pub sha: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentData {
    pub frontmatter: serde_json::Value,
    pub body: String,
}
