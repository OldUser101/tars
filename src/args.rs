use anyhow::{Result, anyhow};
use clap::{
    Arg, ArgAction, Command,
    builder::styling::{AnsiColor, Effects, Styles},
};

pub const DEFAULT_TARS_CONFIG_FILE: &str = "tars.toml";

pub struct Args {
    pub subcommand: TarsSubcommand,
}

pub enum TarsSubcommand {
    Init(InitArgs),
    Build(BuildArgs),
    Clean(CleanArgs),
    Serve(ServeArgs),
    Plugin(PluginArgs),
}

pub struct InitArgs {
    pub force: bool,
    pub path: String,
}

pub struct BuildArgs {
    pub config: String,
    pub no_verify: bool,
}

pub struct CleanArgs {
    pub config: String,
}

pub struct ServeArgs {
    pub config: String,
}

pub struct PluginArgs {
    pub subcommand: PluginSubcommand,
}

pub enum PluginSubcommand {
    List(PluginListArgs),
    Hash(PluginHashArgs),
    Verify(PluginVerifyArgs),
}

pub struct PluginListArgs {
    pub config: String,
}

pub struct PluginVerifyArgs {
    pub config: String,
}

pub struct PluginHashArgs {
    pub name: String,
    pub plugin_dir: String,
}

impl Default for InitArgs {
    fn default() -> Self {
        Self {
            force: false,
            path: ".".to_string(),
        }
    }
}

impl Default for BuildArgs {
    fn default() -> Self {
        Self {
            config: DEFAULT_TARS_CONFIG_FILE.to_string(),
            no_verify: false,
        }
    }
}

impl Default for CleanArgs {
    fn default() -> Self {
        Self {
            config: DEFAULT_TARS_CONFIG_FILE.to_string(),
        }
    }
}

impl Default for ServeArgs {
    fn default() -> Self {
        Self {
            config: DEFAULT_TARS_CONFIG_FILE.to_string(),
        }
    }
}

impl Default for PluginListArgs {
    fn default() -> Self {
        Self {
            config: DEFAULT_TARS_CONFIG_FILE.to_string(),
        }
    }
}

impl Default for PluginVerifyArgs {
    fn default() -> Self {
        Self {
            config: DEFAULT_TARS_CONFIG_FILE.to_string(),
        }
    }
}

fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::BrightGreen.on_default().effects(Effects::BOLD))
        .usage(AnsiColor::BrightGreen.on_default().effects(Effects::BOLD))
        .literal(AnsiColor::BrightCyan.on_default().effects(Effects::BOLD))
        .placeholder(AnsiColor::BrightYellow.on_default())
        .valid(AnsiColor::BrightGreen.on_default())
        .invalid(AnsiColor::BrightRed.on_default())
}

fn build_cli() -> Command {
    Command::new("tars")
        .about("A small, fast, static site generator")
        .version(env!("CARGO_PKG_VERSION"))
        .styles(styles())
        .subcommand(
            Command::new("init")
                .args([
                    Arg::new("force")
                        .short('f')
                        .long("force")
                        .action(ArgAction::SetTrue)
                        .help("Allow overwriting of existing files/directories"),
                    Arg::new("path")
                        .short('p')
                        .long("path")
                        .value_name("PATH")
                        .default_value(".")
                        .required(false)
                        .help("Create a project in the directory specified by PATH"),
                ])
                .about("Initialize a new project"),
        )
        .subcommand(
            Command::new("build")
                .arg(
                    Arg::new("config")
                        .long("config")
                        .value_name("CONFIG")
                        .required(false)
                        .default_value(DEFAULT_TARS_CONFIG_FILE)
                        .help("Specify the configuration file to use"),
                )
                .arg(
                    Arg::new("no_verify")
                        .long("no-verify")
                        .action(ArgAction::SetTrue)
                        .help("Skip hash verification of plugins"),
                )
                .about("Build the project in the current directory"),
        )
        .subcommand(
            Command::new("clean")
                .arg(
                    Arg::new("config")
                        .long("config")
                        .value_name("CONFIG")
                        .required(false)
                        .default_value(DEFAULT_TARS_CONFIG_FILE)
                        .help("Specify the configuration file to use"),
                )
                .about("Clean build outputs"),
        )
        .subcommand(
            Command::new("serve")
                .arg(
                    Arg::new("config")
                        .long("config")
                        .value_name("CONFIG")
                        .required(false)
                        .default_value(DEFAULT_TARS_CONFIG_FILE)
                        .help("Specify the configuration file to use"),
                )
                .about("Serve generated files"),
        )
        .subcommand(
            Command::new("plugin")
                .styles(styles())
                .subcommand(
                    Command::new("list")
                        .arg(
                            Arg::new("config")
                                .long("config")
                                .value_name("CONFIG")
                                .required(false)
                                .default_value(DEFAULT_TARS_CONFIG_FILE)
                                .help("Specify the configuration file to use"),
                        )
                        .about("List installed plugins"),
                )
                .subcommand(
                    Command::new("verify")
                        .arg(
                            Arg::new("config")
                                .long("config")
                                .value_name("CONFIG")
                                .required(false)
                                .default_value(DEFAULT_TARS_CONFIG_FILE)
                                .help("Specify the configuration file to use"),
                        )
                        .about("Verify plugin configuration"),
                )
                .subcommand(
                    Command::new("hash")
                        .arg(Arg::new("name").required(true).help("Plugin name"))
                        .arg(
                            Arg::new("plugin_dir")
                                .long("dir")
                                .value_name("DIR")
                                .required(false)
                                .default_value("plugin")
                                .help("Plugin directory to search in"),
                        )
                        .about("Generate a plugin hash"),
                )
                .about("Manage Tars plugins"),
        )
}

pub fn parse_args() -> Result<Args> {
    let cli = build_cli();

    match cli.get_matches().subcommand() {
        Some(("init", args)) => {
            let force = args.get_flag("force");
            let path = args.get_one::<String>("path").unwrap();

            Ok(Args {
                subcommand: TarsSubcommand::Init(InitArgs {
                    force,
                    path: path.clone(),
                }),
            })
        }
        Some(("build", args)) => {
            let config = args.get_one::<String>("config").unwrap();
            let no_verify = args.get_flag("no_verify");

            Ok(Args {
                subcommand: TarsSubcommand::Build(BuildArgs {
                    config: config.clone(),
                    no_verify,
                }),
            })
        }
        Some(("clean", args)) => {
            let config = args.get_one::<String>("config").unwrap();

            Ok(Args {
                subcommand: TarsSubcommand::Clean(CleanArgs {
                    config: config.clone(),
                }),
            })
        }
        Some(("serve", args)) => {
            let config = args.get_one::<String>("config").unwrap();

            Ok(Args {
                subcommand: TarsSubcommand::Serve(ServeArgs {
                    config: config.clone(),
                }),
            })
        }
        Some(("plugin", args)) => Ok(Args {
            subcommand: TarsSubcommand::Plugin(PluginArgs {
                subcommand: match args.subcommand() {
                    Some(("list", args)) => {
                        let config = args.get_one::<String>("config").unwrap();

                        PluginSubcommand::List(PluginListArgs {
                            config: config.clone(),
                        })
                    }
                    Some(("hash", args)) => {
                        let plugin_dir = args.get_one::<String>("plugin_dir").unwrap();
                        let name = args.get_one::<String>("name").unwrap();

                        PluginSubcommand::Hash(PluginHashArgs {
                            name: name.clone(),
                            plugin_dir: plugin_dir.clone(),
                        })
                    }
                    Some(("verify", args)) => {
                        let config = args.get_one::<String>("config").unwrap();

                        PluginSubcommand::Verify(PluginVerifyArgs {
                            config: config.clone(),
                        })
                    }
                    _ => {
                        return Err(anyhow!("no subcommand specified"));
                    }
                },
            }),
        }),
        _ => Err(anyhow!("no subcommand specified")),
    }
}
