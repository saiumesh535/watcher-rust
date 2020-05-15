use std::{time, thread::sleep};
use std::{collections::HashMap, fs::metadata};
use std::{path::PathBuf, sync::mpsc::Sender};

pub fn watch_for_file(sx: Sender<()>, paths: Vec<PathBuf>) {
    let mut watch_map: HashMap<&PathBuf, u64> = HashMap::new();
    for path in &paths {
        watch_map.insert(path, metadata(path).unwrap().len());
    };
    loop  {
        sleep(time::Duration::from_secs(1));
        for (path, _) in watch_map.clone() {
            let meta = metadata(path).unwrap();
            if watch_map.get(path) != Some(&meta.len()) {
                *watch_map.get_mut(path).unwrap() = meta.len();
                sx.send(()).unwrap();
            }
        }
    };
}