use copy_dir::copy_dir;
use std::{
    fs::{create_dir_all, remove_dir_all, write},
    path::Path,
};
use walkdir::WalkDir;

use crate::{markdown::Page, template::TemplateEnvironment};

pub mod markdown;
pub mod template;

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

    let mut tmpl_env = TemplateEnvironment::new();
    tmpl_env.load_templates().unwrap();

    for entry in WalkDir::new(src_root) {
        let entry = entry.unwrap();
        let entry_type = entry.file_type();
        let path = entry.path();
        let rel_path = path.strip_prefix(src_root).unwrap();
        let mut dst_path = dst_root.join(rel_path);

        if entry_type.is_file() {
            let page = Page::from_file(&path.to_path_buf()).unwrap();

            dst_path.set_extension("html");

            if let Some(tmpl_name) = &page.meta.template {
                let render_str = tmpl_env.render_template(&page, tmpl_name).unwrap();
                write(&dst_path, render_str).unwrap();
            } else {
                write(&dst_path, page.content).unwrap();
            }

            println!("Generated {}", dst_path.display());
        } else if entry_type.is_dir() {
            create_dir_all(dst_path).unwrap();
        }
    }
}
