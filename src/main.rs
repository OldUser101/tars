use crate::{build::Builder, config::Config};

pub mod build;
pub mod config;
pub mod markdown;
pub mod template;

fn main() {
    let config = Config::from_file("tars.toml").unwrap_or_default();
    let mut builder = Builder::new(&config);

    if let Err(e) = builder.build() {
        println!("{e}");
    }
}
