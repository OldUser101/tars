use pulldown_cmark::{Options, Parser};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use chrono::NaiveDate;

#[derive(Debug, Deserialize, Default)]
pub struct FrontMatter {
    title: Option<String>,
    date: Option<NaiveDate>,
    author: Option<String>,
    draft: bool,
    template: Option<String>,
    tags: Option<Vec<String>>,
    slug: Option<String>,
    summary: Option<String>,
    cover_image: Option<String>,
}

pub struct Page {
    pub path: PathBuf,
    pub frontmatter: FrontMatter,
    pub content: String,
}

/// Split frontmatter metadata from Markdown content
fn split_frontmatter(content: &str) -> (FrontMatter, &str) {
    let content = content.trim_start();
    if content.starts_with("---") {
        if let Some(end) = content[3..].find("---") {
            let fm_str = &content[3..3 + end];
            let body = &content[3 + end + 3..];
            let fm = serde_yaml::from_str(fm_str).unwrap_or_default();
            return (fm, body);
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
        frontmatter,
        content: content.to_string(),
    })
}
