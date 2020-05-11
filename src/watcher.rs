use std::{time, thread::sleep};
use std::{collections::HashMap, fs::metadata};
use std::sync::mpsc::Sender;

pub fn watch_for_file(sx: Sender<()>) {
    let mut watch_map: HashMap<&str, u64> = HashMap::new();
    let file_name = "main.go";
    let meta = metadata(file_name).unwrap();
    watch_map.insert(file_name, meta.len());
    loop  {
        sleep(time::Duration::from_secs(1));
        let meta = metadata(file_name).unwrap();
        if watch_map.get(file_name) != Some(&meta.len()) {
            // println!("changed");
            *watch_map.get_mut(file_name).unwrap() = meta.len();
            sx.send(()).unwrap();
        }
    };
}