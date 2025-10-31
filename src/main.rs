use copy_dir::copy_dir;
use std::{
    fs::{create_dir_all, remove_dir_all, write},
    path::Path,
};
use walkdir::WalkDir;

use crate::{
    markdown::Page,
    template::{TemplateContext, TemplateEnvironment},
};

pub mod markdown;
pub mod template;

fn main() {
    let src_root = Path::new("content");
    let static_root = Path::new("static");
    let dst_root = Path::new("build");

    // Clean any existing build
    if dst_root.is_dir() {
        remove_dir_all(dst_root).unwrap();
    }

    create_dir_all(dst_root).unwrap();

    // Copy the static content directory
    if static_root.is_dir() {
        copy_dir(static_root, dst_root.join("static")).unwrap();
        println!("Copied static content directory");
    }

    let mut tmpl_env = TemplateEnvironment::new();
    tmpl_env.load_templates().unwrap();

    let mut pages = Vec::new();

    for entry in WalkDir::new(src_root) {
        let entry = entry.unwrap();
        let entry_type = entry.file_type();
        let path = entry.path();
        let rel_path = path.strip_prefix(src_root).unwrap();
        let dst_path = dst_root.join(rel_path);

        if entry_type.is_file() {
            let page = Page::from_file(src_root, &path.to_path_buf()).unwrap();
            pages.push(page);
        } else if entry_type.is_dir() {
            create_dir_all(dst_path).unwrap();
        }
    }

    for i in 0..pages.len() {
        let page = &pages[i];
        let ctx = TemplateContext::new(&pages, page);

        let mut dst_path = dst_root.join(&page.rel_path);
        dst_path.set_extension("html");

        if let Some(tmpl_name) = &page.meta.template {
            let render_str = tmpl_env.render_template(&ctx, tmpl_name).unwrap();
            write(&dst_path, render_str).unwrap();
        } else {
            write(&dst_path, &page.content).unwrap();
        }

        println!("Generated {}", dst_path.display());
    }
}
