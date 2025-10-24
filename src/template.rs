use std::{fs, path::Path};
use walkdir::WalkDir;

use crate::markdown::Page;

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
    pub fn load_templates(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let tmpl_root = Path::new("template");

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

        Ok(())
    }

    /// Render a template given context and name.
    pub fn render_template(
        &self,
        context: &Page,
        tmpl_name: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let tmpl = self.env.get_template(tmpl_name)?;
        let render_str = tmpl.render(context)?;
        Ok(render_str)
    }
}
