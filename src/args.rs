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
}

pub struct InitArgs {
    pub force: bool,
    pub path: String,
}

pub struct BuildArgs {
    pub config: String,
}

pub struct CleanArgs {
    pub config: String,
}

pub struct ServeArgs {
    pub config: String,
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

            Ok(Args {
                subcommand: TarsSubcommand::Build(BuildArgs {
                    config: config.clone(),
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
        _ => {
            let mut cmd = build_cli();
            cmd.print_help()?;
            println!();
            Err(anyhow!("no subcommand specified"))
        }
    }
}
