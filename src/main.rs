use copy_dir::copy_dir;
use std::{
    fs::{create_dir_all, remove_dir_all, write},
    path::Path,
};
use walkdir::WalkDir;

pub mod markdown;

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
            let page = markdown::parse_content_file(&path.to_path_buf()).unwrap();

            println!("{:#?}", page.frontmatter);

            dst_path.set_extension("html");

            write(&dst_path, page.content).unwrap();

            println!("Generated {}", dst_path.display());
        } else if entry_type.is_dir() {
            create_dir_all(dst_path).unwrap();
        }
    }
}
