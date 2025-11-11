use std::net::SocketAddr;
use anyhow::Result;

use crate::config::Config;

pub async fn run_server(config: &Config) -> Result<()> {
    let socket_str = format!("{}:{}", config.serve.host, config.serve.port);
    let socket_addr: SocketAddr = socket_str.parse()?;

    println!("Running server on {socket_addr}");

    warp::serve(warp::fs::dir(config.build.build_dir.clone()))
        .run(socket_addr)
        .await;

    Ok(())
}
