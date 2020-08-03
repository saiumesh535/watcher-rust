use std::{collections::HashMap, fs::metadata};
use std::{path::PathBuf, sync::mpsc::Sender};
use std::{thread::sleep, time};
use crate::{types::Config, fs_helper::get_paths};

fn get_watch_map(paths: Vec<PathBuf>) -> HashMap<PathBuf, u64> {
    let mut watch_map: HashMap<PathBuf, u64> = HashMap::new();
    for path in paths {
        watch_map.insert(path.clone(), metadata(path).unwrap().len());
    }
    watch_map
}


pub fn watch_for_file(sx: Sender<()>, paths: Vec<PathBuf>, config: &Config) {
    let mut watch_map = get_watch_map(paths);
    loop {
        sleep(time::Duration::from_secs(1));
        // also look for newly added folder/files
        let new_paths = get_paths(&config);
        if new_paths.len() != watch_map.len() {
            sx.send(()).unwrap();
            watch_map = get_watch_map(new_paths);
            continue;
        }
        for (path, _) in watch_map.clone() {
            let meta = metadata(path.clone()).unwrap();
            if watch_map.get(&path) != Some(&meta.len()) {
                *watch_map.get_mut(&path).unwrap() = meta.len();
                sx.send(()).unwrap();
            }
        }
    }
}
