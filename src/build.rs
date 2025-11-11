use anyhow::Result;
use copy_dir::copy_dir;
use std::{
    fs::{create_dir_all, remove_dir_all, write},
    path::Path,
};
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
    content_root: &'a Path,
    static_root: &'a Path,
    build_root: &'a Path,
    built: bool,
}

impl<'a> Builder<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            template_env: TemplateEnvironment::new(),
            pages: Vec::new(),
            config,
            content_root: Path::new(&config.build.content_dir),
            static_root: Path::new(&config.build.static_dir),
            build_root: Path::new(&config.build.build_dir),
            built: false,
        }
    }

    pub fn clean(&self) -> Result<()> {
        // Clean any existing build
        if self.build_root.is_dir() {
            remove_dir_all(self.build_root)?;
        }

        Ok(())
    }

    pub fn copy_static(&self) -> Result<()> {
        // Copy the static content directory
        if self.static_root.is_dir() {
            copy_dir(
                self.static_root,
                self.build_root.join(&self.config.build.static_dir),
            )?;
            println!("Copied static content directory");
        }

        Ok(())
    }

    pub fn load_pages(&mut self) -> Result<()> {
        for entry in WalkDir::new(self.content_root) {
            let entry = entry?;
            let entry_type = entry.file_type();
            let path = entry.path();
            let rel_path = path.strip_prefix(self.content_root)?;
            let dst_path = self.build_root.join(rel_path);

            if entry_type.is_file() {
                let page = Page::from_file(self.content_root, &path.to_path_buf())?;
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

            if !self.config.build.drafts && page.meta.draft {
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

    pub fn rebuild(&mut self) -> Result<()> {
        self.built = false;
        self.template_env = TemplateEnvironment::new();
        self.pages = Vec::new();

        self.build()
    }

    pub fn build(&mut self) -> Result<()> {
        self.clean()?;

        create_dir_all(self.build_root)?;

        self.copy_static()?;

        self.template_env.load_templates(self.config)?;
        self.load_pages()?;
        self.generate_pages()?;

        self.built = true;

        Ok(())
    }
}
