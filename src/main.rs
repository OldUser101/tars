use copy_dir::copy_dir;
use pulldown_cmark::{Options, Parser};
use serde::Deserialize;
use std::{
    fs::{create_dir_all, read_to_string, remove_dir_all, write},
    path::Path,
};
use walkdir::WalkDir;

#[derive(Debug, Deserialize, Default)]
pub struct FrontMatter {
    title: Option<String>,
}

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

fn main() {
    let src_root = Path::new("content");
    let static_root = Path::new("static");
    let dst_root = Path::new("build");

    // Clean any existing build
    if dst_root.is_dir() {
        remove_dir_all(dst_root).unwrap();
        create_dir_all(dst_root).unwrap();
    }

    // Copy the static content directory
    if static_root.is_dir() {
        copy_dir(static_root, dst_root.join("static")).unwrap();
        println!("Copied static content directory");
    }

    for entry in WalkDir::new(src_root) {
        let entry = entry.unwrap();
        let entry_type = entry.file_type();
        let path = entry.path();
        let rel_path = path.strip_prefix(src_root).unwrap();
        let mut dst_path = dst_root.join(rel_path);

        if entry_type.is_file() {
            let content = read_to_string(path).unwrap();
            let (fm, body) = split_frontmatter(&content);

            println!("{:#?}", fm);

            let mut options = Options::empty();
            options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
            let parser = Parser::new_ext(body, options);

            let mut html_output = String::new();
            pulldown_cmark::html::push_html(&mut html_output, parser);

            dst_path.set_extension("html");

            write(&dst_path, html_output).unwrap();

            println!("Generated {}", dst_path.display());
        } else if entry_type.is_dir() {
            create_dir_all(dst_path).unwrap();
        }
    }
}
