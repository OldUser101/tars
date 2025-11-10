use std::{fs, path::{Path, PathBuf}};
use minijinja::value::Value;
use walkdir::WalkDir;
use serde::Serialize;

use crate::{config::Config, markdown::{FrontMatter, Page}};

#[derive(Serialize)]
pub struct TemplateContext<'a> {
    pub pages: &'a [Page],
    pub meta: &'a FrontMatter,
    pub content: &'a String,
    pub path: &'a PathBuf,
}

impl<'a> TemplateContext<'a> {
    /// Create a new `TemplateContext` object given a page list
    /// and current page.
    pub fn new(pages: &'a Vec<Page>, page: &'a Page) -> Self {
        Self {
            pages,
            meta: &page.meta,
            content: &page.content,
            path: &page.rel_path,
        }
    }
}

pub struct TemplateEnvironment<'a> {
    env: minijinja::Environment<'a>,
}

impl<'a> TemplateEnvironment<'a> {
    /// Returns a new, empty, template environment.
    pub fn new() -> Self {
        Self {
            env: minijinja::Environment::new(),
        }
    }

    /// Load all templates from the templates directory.
    ///
    /// This function succeeds if all templates are valid, or if the `templates`
    /// directory doesn't exist.
    pub fn load_templates(&mut self, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
        let tmpl_root = Path::new(&config.build.template_dir);

        // If the templates directory doesn't exist, don't try to load anything.
        if !tmpl_root.is_dir() {
            return Ok(());
        }

        for entry in WalkDir::new(tmpl_root) {
            let entry = entry?;
            let path = entry.path();

            // Get the path inside `templates`
            let name = path.strip_prefix(tmpl_root)?.to_string_lossy().into_owned();

            if path.is_file() {
                let tmpl_str = fs::read_to_string(path)?;

                self.env.add_template_owned(name.clone(), tmpl_str)?;

                println!("Loaded template {name}");
            }
        }

        self.env.add_global("site", Value::from_serialize(&config.site));
        self.env.add_global("extra", Value::from_serialize(&config.extra));

        Ok(())
    }

    /// Render a template given context and name.
    pub fn render_template(
        &self,
        context: &TemplateContext,
        tmpl_name: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let tmpl = self.env.get_template(tmpl_name)?;
        let render_str = tmpl.render(context)?;
        Ok(render_str)
    }
}
