use std::sync::mpsc::channel;
use std::thread;

mod command;
mod fs_helper;
mod types;
mod watcher;

fn main() {
    let config = fs_helper::read_config("fwr.json");
    let paths = fs_helper::get_paths(&config);
    let (tx, rx) = channel::<()>();
    let cloned_config = config.clone();
    let command_thread = thread::spawn(move || {
        command::run_init_command(rx, cloned_config);
    });
    let watcher_thread = thread::spawn(move || {
        watcher::watch_for_file(tx, paths, &config);
    });
    command_thread.join().unwrap();
    watcher_thread.join().unwrap();
}
