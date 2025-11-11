use anyhow::Result;
use notify::{Event, EventKind, RecursiveMode, Watcher};
use std::net::SocketAddr;
use std::sync::Arc;
use std::{path::Path, sync::mpsc};

use crate::{build::Builder, config::Config};

pub async fn run_server(config: Arc<Config>) -> Result<()> {
    let socket_str = format!("{}:{}", config.serve.host, config.serve.port);
    let socket_addr: SocketAddr = socket_str.parse()?;
    let build_dir = config.build.build_dir.clone();

    let mut builder = Builder::new(&config);
    println!("Building...");
    builder.build()?;

    let (notify_tx, notify_rx) = mpsc::channel::<notify::Result<Event>>();
    let mut watcher = notify::recommended_watcher(notify_tx)?;
    watcher.watch(
        Path::new(&config.build.content_dir),
        RecursiveMode::Recursive,
    )?;
    watcher.watch(
        Path::new(&config.build.template_dir),
        RecursiveMode::Recursive,
    )?;
    watcher.watch(
        Path::new(&config.build.static_dir),
        RecursiveMode::Recursive,
    )?;

    tokio::spawn(async move {
        let mut builder = Builder::new(&config);
        for res in notify_rx {
            match res {
                Ok(event) => match event.kind {
                    EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => {
                        println!("Rebuilding...");
                        if let Err(e) = builder.rebuild() {
                            println!("Build error: {e}");
                        }
                    }
                    _ => {}
                },
                Err(e) => {
                    println!("Watch error: {e}");
                }
            }
        }
    });

    println!("Running server on {socket_addr}");

    warp::serve(warp::fs::dir(build_dir)).run(socket_addr).await;

    Ok(())
}
