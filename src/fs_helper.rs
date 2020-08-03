use crate::types::Config;
use serde_json::from_str;
use std::{
    ffi::OsStr,
    fs::{read_dir, read_to_string},
    path::PathBuf,
};

fn read_file(path: &str) -> String {
    read_to_string(path).unwrap()
}

pub fn read_config(path: &str) -> Config {
    from_str(read_file(path).as_str()).unwrap()
}

fn should_consider_extension(extensions: &Vec<String>, file_extension: Option<&OsStr>) -> bool {
    for extension in extensions {
        if file_extension == Some(OsStr::new(extension.as_str())) {
            return true;
        }
    }
    return false;
}

fn get_last_instance(path: &str) -> String {
    let split = path.split("/").collect::<Vec<&str>>();
    split[split.len() - 1].to_string()
}

fn get_watch_paths(paths: &mut Vec<PathBuf>, src_path: PathBuf, config: &Config) {
    if src_path.is_dir() {
        let dir = read_dir(src_path).unwrap();
        for file in dir {
            let entry = file.unwrap();
            if entry.path().is_dir() {
                // check fot ignore dir
                if config.ignore_folders != None {
                    let last_instance = get_last_instance(entry.path().as_path().to_str().unwrap());
                    if config.ignore_folders.as_ref().unwrap().contains(&last_instance) {
                        continue;
                    }
                }
                get_watch_paths(paths, entry.path(), config);
                continue;
            }
            if should_consider_extension(&config.ext, entry.path().extension()) {
                paths.push(entry.path());
            }
        }
    } else {
        if should_consider_extension(&config.ext, src_path.extension()) {
            paths.push(src_path);
        }
    }
}

pub fn get_paths(config: &Config) -> Vec<PathBuf> {
    let mut paths = vec![];
    let src_path = PathBuf::from(config.dir.as_str());
    get_watch_paths(&mut paths, src_path, config);
    return paths;
}
