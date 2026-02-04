use anyhow::{Result, anyhow};
use std::{collections::HashMap, path::Path, process::exit, sync::Arc};

use crate::{
    args::{DEFAULT_TARS_CONFIG_FILE, InitArgs, PluginSubcommand, TarsSubcommand, parse_args},
    build::Builder,
    config::{Config, HookType, Plugin},
    serve::run_server,
};

pub mod args;
pub mod build;
pub mod config;
pub mod markdown;
pub mod serve;
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

fn load_config(path: &str) -> Config {
    Config::from_file(path).unwrap_or_else(|e| {
        eprintln!("Failed to load config file {path}: {e}");
        exit(1);
    })
}

#[tokio::main]
async fn main() {
    let args = parse_args().unwrap_or_else(|e| {
        println!("{e}");
        exit(1);
    });

    match args.subcommand {
        TarsSubcommand::Init(args) => {
            let config = Config::default();

            if let Err(e) = init_project(&args, &config) {
                println!("Error initializing project: {e}");
                exit(1);
            } else {
                println!("Initialized project in '{}' .", args.path);
            }
        }
        TarsSubcommand::Build(args) => {
            let config = load_config(&args.config);
            let mut builder = Builder::new(&config, args.no_verify, None);

            if let Err(e) = builder.build() {
                println!("{e}");
                exit(1);
            }
        }
        TarsSubcommand::Clean(args) => {
            let config = load_config(&args.config);
            let builder = Builder::new(&config, false, None);

            if let Err(e) = builder.clean() {
                println!("{e}");
                exit(1);
            }
        }
        TarsSubcommand::Serve(args) => {
            let config = load_config(&args.config);

            if let Err(e) = run_server(Arc::new(config)).await {
                println!("{e}");
                exit(1);
            }
        }
        TarsSubcommand::Plugin(args) => match args.subcommand {
            PluginSubcommand::List(args) => {
                let config = load_config(&args.config);

                for p in config.plugins {
                    println!("Name: {}, Hook: {:#?}", p.name, p.hook_type);
                }
            }
            PluginSubcommand::Verify(args) => {
                let config = load_config(&args.config);

                for p in config.plugins {
                    if let Err(e) = p.resolve(Path::new(&config.build.plugin_dir), false) {
                        println!("Verification failed for {}: {}", p.name, e);
                        exit(1);
                    } else {
                        println!("Verification success for {}", p.name);
                    }
                }
            }
            PluginSubcommand::Hash(args) => {
                // The actual values don't matter hare, as long as `name` is correct
                let plugin = Plugin {
                    name: args.name,
                    hook_type: HookType::Pre,
                    hash: "".to_string(),
                    args: HashMap::new(),
                };

                match plugin.get_hash(Path::new(&args.plugin_dir)) {
                    Ok(hash) => {
                        println!("{hash}");
                    }
                    Err(e) => {
                        println!("Error while calculating digest: {e}");
                        exit(1);
                    }
                }
            }
        },
    }
}
