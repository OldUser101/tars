use anyhow::{Result, anyhow};
use std::{path::Path, process::exit};

use crate::{
    args::{DEFAULT_TARS_CONFIG_FILE, InitArgs, TarsSubcommand, parse_args},
    build::Builder,
    config::Config,
};

pub mod args;
pub mod build;
pub mod config;
pub mod markdown;
pub mod template;

fn is_dir_empty(path: &Path) -> std::io::Result<bool> {
    let mut entries = std::fs::read_dir(path)?;
    Ok(entries.next().is_none())
}

fn init_project(args: &InitArgs, config: &Config) -> Result<()> {
    let root_dir = Path::new(&args.path);

    if !root_dir.is_dir() {
        return Err(anyhow!(
            "Directory '{}' does not exist.",
            root_dir.display()
        ));
    }

    let content_dir = root_dir.join(&config.build.content_dir);
    let static_dir = root_dir.join(&config.build.static_dir);
    let template_dir = root_dir.join(&config.build.template_dir);
    let config_file = root_dir.join(DEFAULT_TARS_CONFIG_FILE);

    let dir_empty = is_dir_empty(root_dir).unwrap_or(false);

    if !args.force && !dir_empty {
        return Err(anyhow!(
            "Directory '{}' not empty, or not readable.",
            root_dir.display()
        ));
    }

    std::fs::create_dir_all(content_dir)?;
    std::fs::create_dir_all(static_dir)?;
    std::fs::create_dir_all(template_dir)?;
    std::fs::write(config_file, "")?;

    Ok(())
}

fn main() {
    let args = parse_args().unwrap_or_else(|e| {
        println!("{e}");
        exit(1);
    });

    match args.subcommand {
        TarsSubcommand::Init(args) => {
            let config = Config::default();

            if let Err(e) = init_project(&args, &config) {
                println!("Error initializing project: {e}");
            } else {
                println!("Initialized project in '{}' .", args.path);
            }
        }
        TarsSubcommand::Build(args) => {
            let config = Config::from_file(&args.config).unwrap_or_default();
            let mut builder = Builder::new(&config);

            if let Err(e) = builder.build() {
                println!("{e}");
            }
        }
        TarsSubcommand::Clean(args) => {
            let config = Config::from_file(&args.config).unwrap_or_default();
            let builder = Builder::new(&config);

            if let Err(e) = builder.clean() {
                println!("{e}");
            }
        }
    }
}
