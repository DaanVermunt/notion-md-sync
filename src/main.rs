mod conf;
mod notionlib;

use crate::conf::{conf};
use crate::notionlib::push_to_notion::NotionSyncer;

use std::path::{Path, PathBuf};
use notify::{Watcher, RecursiveMode, Event, RecommendedWatcher, Config};

use notify::event::AccessKind::Close;
use notify::event::AccessMode::Write;
use notify::EventKind::Access;

use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};
use tokio;

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let watcher = RecommendedWatcher::new(
        move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}


async fn async_watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (mut watcher, mut rx) = async_watcher()?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    while let Some(res) = rx.next().await {
        match res {
            Ok(event) => {
                handle_event(event).await;
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    futures::executor::block_on(async {
        if let Err(e) = async_watch(conf().path).await {
            println!("error: {:?}", e)
        }
    });
}

fn relevant_paths(e: Event) -> Option<Vec<PathBuf>> {
    let paths: Vec<PathBuf> = e.paths.clone().into_iter().filter(|path| {
        if let Some(ext) = path.extension() {
            return conf().extensions.iter().any(|e| e.as_str() == ext);
        }

        return false;
    }).collect();

    if paths.len() == 0 || e.kind != Access(Close(Write)) {
        return None;
    }

    return Some(paths);
}

async fn handle_event(e: Event) {
    let paths = relevant_paths(e.clone());

    match paths {
        None => return,
        Some(paths) => {
            for path in paths {
                println!("File changed: {:?}", path);
                let syncer = NotionSyncer::new(conf().token.clone()).await;
                let res = syncer.push_file_to_notion(path).await;
                match res {
                    Ok(_) => println!("Pushed file to notionlib"),
                    Err(e) => println!("Error pushing file to notionlib: {:?}", e),
                }
            }
        }
    }
}
