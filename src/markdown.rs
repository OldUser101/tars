use chrono::NaiveDate;
use pulldown_cmark::{Options, Parser};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct FrontMatter {
    pub title: Option<String>,
    pub date: Option<NaiveDate>,
    pub author: Option<String>,
    pub draft: Option<bool>,
    pub template: Option<String>,
    pub tags: Option<Vec<String>>,
    pub slug: Option<String>,
    pub summary: Option<String>,
    pub cover_image: Option<String>,
}

impl FrontMatter {
    pub fn merge_with_default(&self) -> Self {
        Self {
            title: self.title.clone(),
            date: self.date,
            author: self.author.clone(),
            draft: self.draft.or(Some(false)),
            template: self.template.clone().or(Some("default.html".to_string())),
            tags: self.tags.clone(),
            slug: self.slug.clone(),
            summary: self.summary.clone(),
            cover_image: self.cover_image.clone(),
        }
    }
}

impl Default for FrontMatter {
    fn default() -> Self {
        Self {
            title: None,
            date: None,
            author: None,
            draft: Some(false),
            template: Some("default.html".to_string()),
            tags: None,
            slug: None,
            summary: None,
            cover_image: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    #[serde(skip)]
    pub path: PathBuf,
    pub meta: FrontMatter,
    pub content: String,
}

/// Split frontmatter metadata from Markdown content
fn split_frontmatter(content: &str) -> (FrontMatter, &str) {
    let content = content.trim_start();
    if content.starts_with("---") {
        if let Some(end) = content[3..].find("---") {
            let fm_str = &content[3..3 + end];
            let body = &content[3 + end + 3..];
            let fm: FrontMatter = serde_yaml::from_str(fm_str).unwrap_or_default();
            return (fm.merge_with_default(), body);
        }
    }
    (FrontMatter::default(), content)
}

/// Parse a content file into a `Page` structure
pub fn parse_content_file(path: &PathBuf) -> Result<Page, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let (frontmatter, content) = split_frontmatter(&content);

    let mut options = Options::empty();
    options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
    let parser = Parser::new_ext(content, options);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    Ok(Page {
        path: path.to_path_buf(),
        meta: frontmatter,
        content: html_output.to_string(),
    })
}
