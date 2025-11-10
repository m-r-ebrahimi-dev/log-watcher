use std::env;
use std::path::Path;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: log-watcher <path_to_log_file>");
        return Ok(());
    }
    let path_str = &args[1];
    println!("---> Watching file: {}", path_str);

    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    watcher.watch(Path::new(path_str), RecursiveMode::NonRecursive)?;

    println!("\n---> Waiting for file changes... (Press Ctrl+C to exit)");

    for res in rx {
        match res {
            Ok(event) => {
                if event.kind.is_modify() {
                    println!("-> File modified! Next step: Read and parse.");
                }
            }
            Err(e) => eprintln!("Watch error: {:?}", e),
        }
    }

    Ok(())
}