mod push_to_notion;

use std::path::{Path, PathBuf};
use notify::{Watcher, RecursiveMode, Event, recommended_watcher};
use notify::event::AccessKind::Close;
use notify::event::AccessMode::Write;
use notify::EventKind::Access;

struct SyncConfig {
    extensions: Vec<String>,
    path: String,
}

fn main() {
    let mut watcher = notify::recommended_watcher(|res| {
        match res {
            Ok(event) => handle_event(event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }).unwrap();

    watcher.watch(Path::new(conf().path.as_str()), RecursiveMode::Recursive).unwrap();

    loop {
        // TODO EVERY X MINUTES PULL NOTION FILES
    }
}

fn conf() -> SyncConfig {
    return SyncConfig {
        path: String::from("/home/daan/notes"),
        extensions: Vec::from([String::from("md")]),
    };
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

fn handle_event(e: Event) {
    let paths = relevant_paths(e.clone());

    match paths {
        None => return,
        Some(paths) => {
            for path in paths {
                push_to_notion::push_file_to_notion(path)
            }
        }
    }
}
