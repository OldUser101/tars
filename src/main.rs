use pulldown_cmark::{Options, Parser};
use std::{
    fs::{create_dir_all, read_to_string, remove_dir_all, write},
    path::Path,
};
use walkdir::WalkDir;

fn main() {
    let src_root = Path::new("content");
    let dst_root = Path::new("build");

    // Clean any existing build
    if dst_root.is_dir() {
        remove_dir_all(dst_root).unwrap();
    }

    for entry in WalkDir::new(src_root) {
        let entry = entry.unwrap();
        let entry_type = entry.file_type();
        let path = entry.path();
        let rel_path = path.strip_prefix(src_root).unwrap();
        let mut dst_path = dst_root.join(rel_path);

        if entry_type.is_file() {
            let content = read_to_string(path).unwrap();

            let mut options = Options::empty();
            options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
            let parser = Parser::new_ext(&content, options);

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
