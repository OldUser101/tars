use anyhow::Result;
use notify::{Event, EventKind, RecursiveMode, Watcher};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use std::{path::Path, sync::mpsc};
use warp::Filter;

use crate::{build::Builder, config::Config};

const RELOAD_JS: &str = "
<script>
const es = new EventSource(\"/__tars_reload__\");
es.onmessage = () => location.reload();
</script>
";

pub async fn run_server(config: Arc<Config>) -> Result<()> {
    let socket_str = format!("{}:{}", config.serve.host, config.serve.port);
    let socket_addr: SocketAddr = socket_str.parse()?;
    let build_dir = config.build.build_dir.clone();

    let (refresh_tx, _) = tokio::sync::broadcast::channel::<()>(16);
    let refresh_sse = refresh_tx.clone();
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

    std::thread::spawn(move || {
        let mut builder = Builder::new(&config, false, Some(RELOAD_JS));
        println!("Building...");
        if let Err(e) = builder.build() {
            println!("Build error: {e}");
        }

        for res in notify_rx {
            match res {
                Ok(event) => match event.kind {
                    EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => {
                        println!("Rebuilding...");
                        if let Err(e) = builder.rebuild() {
                            println!("Build error: {e}");
                        }
                        let _ = refresh_tx.send(());
                    }
                    _ => {}
                },
                Err(e) => {
                    println!("Watch error: {e}");
                }
            }
        }
    });

    println!("Running server on http://{socket_addr}");

    let sse = warp::path("__tars_reload__").and(warp::get()).map(move || {
        let mut rx = refresh_sse.subscribe();

        let stream = async_stream::stream! {
            loop {
                if rx.recv().await.is_ok() {
                    yield Ok::<_, Infallible>(
                        warp::sse::Event::default().data("reload")
                    );
                }
            }
        };

        warp::sse::reply(warp::sse::keep_alive().stream(stream))
    });

    let routes = warp::fs::dir(build_dir).or(sse);
    warp::serve(routes).run(socket_addr).await;

    Ok(())
}
