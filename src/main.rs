use std::thread;
use std::sync::mpsc::channel;

mod command;
mod watcher;
mod types;
mod fs_helper;

fn main() {
    let config = fs_helper::read_config("fwr.json");
    let paths = fs_helper::get_paths(&config);
    let (tx, rx) = channel::<()>();
    let command_thread = thread::spawn(move || {
        command::run_init_command(rx, config);
    });
    let watcher_thread = thread::spawn(|| {
        watcher::watch_for_file(tx, paths);
    });
    command_thread.join().unwrap();
    watcher_thread.join().unwrap();
}
