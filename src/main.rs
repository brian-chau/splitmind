extern crate notify;

use notify::{Config, Error, RecommendedWatcher, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;

// this code is similar to how windows tree /f /a works
// but simplified for demonstration purposes
fn draw_tree(dir: &Path, prefix: &str) {
    if dir.is_dir() {
        let entries: Vec<_> = std::fs::read_dir(dir)
            .unwrap()
            .map(|res| res.unwrap())
            .collect();
        let count = entries.len();
        for (i, entry) in entries.iter().enumerate() {
            let path = entry.path();
            let is_last = i == count - 1;
            let new_prefix = if is_last {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };
            if path.is_dir() {
                println!("{}└── {}", prefix, entry.file_name().to_string_lossy());
                draw_tree(&path, &new_prefix);
            } else {
                println!("{}└── {}", prefix, entry.file_name().to_string_lossy());
            }
        }
    }
}

fn main() {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut w: Result<RecommendedWatcher, Error> = Watcher::new(
        tx,
        Config::default().with_poll_interval(std::time::Duration::from_secs(2)),
    );

    draw_tree(Path::new("."), "");

    match w {
        Ok(ref mut watcher) => {
            // Add a path to be watched. All files and directories at that path and
            // below will be monitored for changes.
            watcher
                .watch(Path::new("."), notify::RecursiveMode::Recursive)
                .unwrap();

            loop {
                match rx.recv() {
                    Ok(event) => {
                        let e1 = event.unwrap().clone();

                        // This pre-check determines if the changed file is in the .git folder
                        // We want to ignore any changes in any .git folder
                        if e1.paths.iter().any(|p| p.components().any(|c| c.as_os_str() == ".git")) {
                            continue;
                        }

                        // We also want to skip if the event is just a open or modify event
                        if e1.kind.is_access() || e1.kind.is_modify() {
                            continue;
                        }

                        // println!("Change detected: {:?}", e2);
                        // println!("Current directory tree:");
                        // Clear the terminal screen
                        print!("\x1B[2J\x1B[1;1H");
                        draw_tree(Path::new("."), "");
                    }
                    Err(e) => {
                        println!("watch error: {:?}", e)
                    }
                }
            }
        }
        Err(_) => println!("Error"),
    }
}
