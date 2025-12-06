use anyhow::{Result, anyhow};
use fs_extra::{
    dir::{CopyOptions as DirOptions, copy as copy_dir},
    file::{CopyOptions as FileOptions, copy as copy_file},
};
use std::{
    fs::{create_dir_all, remove_dir_all, write},
    path::{Path, PathBuf},
};
use tempfile::{TempDir, tempdir};
use walkdir::WalkDir;

use crate::{
    config::Config,
    markdown::Page,
    template::{TemplateContext, TemplateEnvironment},
};

pub struct Builder<'a> {
    template_env: TemplateEnvironment<'a>,
    pages: Vec<Page>,
    config: &'a Config,
    content_root: PathBuf,
    static_root: PathBuf,
    build_root: PathBuf,
    template_root: PathBuf,
    config_path: PathBuf,
    tmp_dir: Option<TempDir>,
    built: bool,
}

impl<'a> Builder<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            template_env: TemplateEnvironment::new(),
            pages: Vec::new(),
            config,
            content_root: PathBuf::new(),
            static_root: PathBuf::new(),
            build_root: PathBuf::new(),
            template_root: PathBuf::new(),
            config_path: PathBuf::new(),
            tmp_dir: None,
            built: false,
        }
    }

    pub fn clean(&self) -> Result<()> {
        // Clean any existing build
        if self.build_root.is_dir() {
            remove_dir_all(&self.build_root)?;
        }

        Ok(())
    }

    pub fn copy_static(&self) -> Result<()> {
        // Copy the static content directory
        if self.static_root.is_dir() {
            let static_dst = self.build_root.join(&self.config.build.static_prefix);
            if !static_dst.is_dir() {
                create_dir_all(&static_dst)?;
            }

            for entry in std::fs::read_dir(&self.static_root)? {
                let entry = entry?;
                let src_path = entry.path();
                let dst_path = static_dst.join(entry.file_name());

                if src_path.is_dir() {
                    let mut options = DirOptions::new();
                    options.overwrite = true;
                    options.copy_inside = true;

                    copy_dir(&src_path, &dst_path, &options)?;
                } else if src_path.is_file() {
                    let mut options = FileOptions::new();
                    options.overwrite = true;

                    copy_file(&src_path, &dst_path, &options)?;
                }
            }

            println!(
                "Copied static content directory to {}",
                static_dst.display()
            );
        }

        Ok(())
    }

    pub fn load_pages(&mut self) -> Result<()> {
        for entry in WalkDir::new(&self.content_root) {
            let entry = entry?;
            let entry_type = entry.file_type();
            let path = entry.path();
            let rel_path = path.strip_prefix(&self.content_root)?;
            let dst_path = self.build_root.join(rel_path);

            if entry_type.is_file() {
                let page = Page::from_file(self.config, &self.content_root, &path.to_path_buf())?;
                self.pages.push(page);
            } else if entry_type.is_dir() {
                create_dir_all(dst_path)?;
            }
        }

        Ok(())
    }

    pub fn generate_pages(&mut self) -> Result<()> {
        for i in 0..self.pages.len() {
            let page = &self.pages[i];

            if !self.config.build.include_drafts && page.meta.draft {
                continue;
            }

            let ctx = TemplateContext::new(&self.pages, page);

            let mut dst_path = self.build_root.join(&page.rel_path);
            dst_path.set_extension("html");

            if let Some(tmpl_name) = &page.meta.template {
                let render_str = self.template_env.render_template(&ctx, tmpl_name)?;
                write(&dst_path, render_str)?;
            } else {
                write(&dst_path, &page.content)?;
            }

            println!("Generated {}", dst_path.display());
        }

        Ok(())
    }

    fn prepare_build_dir(&mut self) -> Result<()> {
        let tmp_root = tempdir()?;

        println!("Build directory: {}", tmp_root.path().display());

        fn push_and_copy(src: &Path, dst: &mut PathBuf, kind: &str) -> Result<()> {
            let name = src
                .file_name()
                .ok_or_else(|| anyhow!("{} path must be a valid directory", kind))?;
            dst.push(name);

            let mut options = DirOptions::new();
            options.overwrite = true;
            options.copy_inside = true;

            if src.is_dir() {
                copy_dir(src, dst, &options)?;
            }
            Ok(())
        }

        let content_dir = Path::new(&self.config.build.content_dir);
        let static_dir = Path::new(&self.config.build.static_dir);
        let template_dir = Path::new(&self.config.build.template_dir);

        self.content_root.push(tmp_root.path());
        self.static_root.push(tmp_root.path());
        self.template_root.push(tmp_root.path());

        self.build_root.push(tmp_root.path());
        self.build_root.push(&self.config.build.build_dir);

        push_and_copy(content_dir, &mut self.content_root, "content")?;
        push_and_copy(static_dir, &mut self.static_root, "static")?;
        push_and_copy(template_dir, &mut self.template_root, "template")?;

        let mut config_path = PathBuf::from(tmp_root.path());
        config_path.push("tars.toml");

        self.config_path = config_path;

        let mut options = FileOptions::new();
        options.overwrite = true;

        copy_file(&self.config.path, &self.config_path, &options)?;

        self.tmp_dir = Some(tmp_root);

        Ok(())
    }

    fn copy_generated(&self) -> Result<()> {
        let build_dst = Path::new(&self.config.build.build_dir);

        let mut options = DirOptions::new();
        options.overwrite = true;
        options.copy_inside = true;
        options.content_only = true;

        copy_dir(&self.build_root, build_dst, &options)?;

        Ok(())
    }

    pub fn rebuild(&mut self) -> Result<()> {
        self.built = false;
        self.template_env = TemplateEnvironment::new();
        self.pages = Vec::new();

        self.build()
    }

    pub fn build(&mut self) -> Result<()> {
        self.prepare_build_dir()?;
        self.clean()?;

        create_dir_all(&self.build_root)?;

        self.copy_static()?;

        self.template_env.load_templates(self.config)?;
        self.load_pages()?;
        self.generate_pages()?;

        self.copy_generated()?;

        self.built = true;

        Ok(())
    }
}
