use crate::models::{BlogPost, Project};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct FrontmatterBlock {
    pub raw: String,
    pub start_line: usize,
    pub end_line: usize,
}

impl FrontmatterBlock {
    pub fn new() -> Self {
        Self {
            raw: String::new(),
            start_line: 0,
            end_line: 0,
        }
    }
}

pub fn extract_frontmatter(lines: &[String]) -> Option<FrontmatterBlock> {
    if lines.is_empty() || lines[0].trim() != "---" {
        return None;
    }
    let mut end = 1;
    while end < lines.len() && lines[end].trim() != "---" {
        end += 1;
    }
    if end >= lines.len() {
        return None;
    }
    let raw = lines[1..end].join("\n");
    Some(FrontmatterBlock {
        raw,
        start_line: 0,
        end_line: end,
    })
}

pub fn serialize_frontmatter(value: &Value) -> Result<String, String> {
    let mapping = json_to_yaml_mapping(value)?;
    let mut out = String::new();
    for (key, val) in &mapping {
        out.push_str(&format!("{}: {}\n", key, val));
    }
    Ok(out)
}

fn json_to_yaml_mapping(value: &Value) -> Result<Vec<(String, String)>, String> {
    match value {
        Value::Object(map) => {
            let mut pairs = Vec::new();
            for (key, val) in map {
                let yaml_val = json_val_to_yaml(val)?;
                pairs.push((key.clone(), yaml_val));
            }
            Ok(pairs)
        }
        _ => Err("expected object".to_string()),
    }
}

fn json_val_to_yaml(val: &Value) -> Result<String, String> {
    match val {
        Value::String(s) => Ok(s.clone()),
        Value::Bool(b) => Ok(b.to_string()),
        Value::Number(n) => Ok(n.to_string()),
        Value::Array(arr) => {
            let items: Vec<String> = arr
                .iter()
                .map(|v| json_val_to_yaml(v))
                .collect::<Result<Vec<_>, _>>()?;
            if items.is_empty() {
                Ok("[]".to_string())
            } else {
                Ok(format!("[{}]", items.join(", ")))
            }
        }
        Value::Object(obj) => {
            let items: Vec<String> = obj
                .iter()
                .map(|(k, v)| {
                    let v = json_val_to_yaml(v)?;
                    Ok(format!("{}: {}", k, v))
                })
                .collect::<Result<Vec<_>, String>>()?;
            Ok(format!("{{{}}}", items.join(", ")))
        }
        Value::Null => Ok("~".to_string()),
    }
}

pub fn validate_blog_frontmatter(value: &Value) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    let obj = match value.as_object() {
        Some(o) => o,
        None => {
            errors.push("frontmatter must be an object".to_string());
            return Err(errors);
        }
    };

    if !obj.contains_key("title") {
        errors.push("missing required field: title".to_string());
    }
    if !obj.contains_key("date") {
        errors.push("missing required field: date".to_string());
    }
    if !obj.contains_key("description") {
        errors.push("missing required field: description".to_string());
    }

    validate_common(obj, &mut errors);

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub fn validate_project_frontmatter(value: &Value) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    let obj = match value.as_object() {
        Some(o) => o,
        None => {
            errors.push("frontmatter must be an object".to_string());
            return Err(errors);
        }
    };

    if !obj.contains_key("title") {
        errors.push("missing required field: title".to_string());
    }
    if !obj.contains_key("date") {
        errors.push("missing required field: date".to_string());
    }
    if !obj.contains_key("description") {
        errors.push("missing required field: description".to_string());
    }

    validate_common(obj, &mut errors);

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn validate_common(obj: &serde_json::Map<String, Value>, errors: &mut Vec<String>) {
    if let Some(date) = obj.get("date") {
        if let Value::String(s) = date {
            if chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").is_err() {
                errors.push("date must be in YYYY-MM-DD format".to_string());
            }
        } else {
            errors.push("date must be a string".to_string());
        }
    }
    if let Some(tags) = obj.get("tags") {
        match tags {
            Value::Array(arr) => {
                for (i, t) in arr.iter().enumerate() {
                    if !t.is_string() {
                        errors.push(format!("tags[{}] must be a string", i));
                    }
                }
            }
            Value::String(_) => {}
            _ => {
                errors.push("tags must be an array or string".to_string());
            }
        }
    }
    if let Some(draft) = obj.get("draft") {
        if !draft.is_boolean() {
            errors.push("draft must be a boolean".to_string());
        }
    }
}

pub fn build_blog_frontmatter(post: &BlogPost) -> Value {
    let mut map = serde_json::Map::new();
    map.insert("title".to_string(), Value::String(post.title.clone()));
    map.insert("date".to_string(), Value::String(post.date.clone()));
    map.insert(
        "description".to_string(),
        Value::String(post.description.clone()),
    );

    let tags: Vec<Value> = post.tags.iter().map(|t| Value::String(t.clone())).collect();
    map.insert("tags".to_string(), Value::Array(tags));
    map.insert("draft".to_string(), Value::Bool(post.draft));
    if let Some(ref img) = post.image {
        map.insert("image".to_string(), Value::String(img.clone()));
    }
    Value::Object(map)
}

pub fn build_project_frontmatter(project: &Project) -> Value {
    let mut map = serde_json::Map::new();
    map.insert("title".to_string(), Value::String(project.title.clone()));
    map.insert(
        "description".to_string(),
        Value::String(project.description.clone()),
    );
    map.insert("date".to_string(), Value::String(project.date.clone()));

    let tech: Vec<Value> = project
        .tech_stack
        .iter()
        .map(|t| Value::String(t.clone()))
        .collect();
    map.insert("techStack".to_string(), Value::Array(tech));
    map.insert("featured".to_string(), Value::Bool(project.featured));

    let mut links = serde_json::Map::new();
    if let Some(ref s) = project.links.source {
        links.insert("source".to_string(), Value::String(s.clone()));
    }
    if let Some(ref d) = project.links.demo {
        links.insert("demo".to_string(), Value::String(d.clone()));
    }
    map.insert("links".to_string(), Value::Object(links));

    Value::Object(map)
}
