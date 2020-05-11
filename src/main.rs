use std::thread;
use std::sync::mpsc::channel;

mod command;
mod watcher;

fn main() {
    let (tx, rx) = channel::<()>();
    let command_thread = thread::spawn(move || {
        command::run_init_command(rx);
    });
    let watcher_thread = thread::spawn(|| {
        watcher::watch_for_file(tx);
    });
    command_thread.join().unwrap();
    watcher_thread.join().unwrap();
}
