use notify::{RecommendedWatcher, Watcher, RecursiveMode};
use notify::DebouncedEvent;
use std::time::Duration;
use std::sync::mpsc;
use std::path::PathBuf;

pub fn watch_patches() -> (RecommendedWatcher, mpsc::Receiver<DebouncedEvent>) {
    let (tx, rx) = mpsc::channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_millis(100)).unwrap();
    watcher.watch("patches", RecursiveMode::Recursive).unwrap();
    (watcher, rx)
}

pub fn get_changed_files<T>(events: T) -> Vec<PathBuf> where
    T: Iterator<Item=DebouncedEvent>
{
    events.filter_map(|event| match event{
        DebouncedEvent::Create(path_buf) => Some(path_buf),
        _ => None
    }).collect()
}