use crate::types::Config;
use std::process::Command;
use std::sync::mpsc::Receiver;
use std::thread;

fn get_pids(port: &str) -> Vec<String> {
    let mut pids: Vec<String> = vec![];
    let port_query = format!("netstat -ano | findstr :{}", port);
    let command = Command::new("cmd.exe")
        .args(&["/C", port_query.as_str()])
        .output();
    if command.is_err() {
        panic!(command.is_err());
    };
    let command_out_put = command.unwrap().stdout;
    let lol = String::from_utf8(command_out_put).unwrap();
    for lol in lol.lines() {
        let lol_string = String::from(lol);
        if lol_string.contains("LISTENING") {
            let mut pid = String::new();
            for ch in lol_string.chars().rev().collect::<String>().chars() {
                let num = ch.to_string().parse::<i32>();
                if !num.is_err() {
                    pid.push_str(num.unwrap().to_string().as_str());
                } else {
                    pids.push(pid.chars().rev().collect());
                    break;
                }
            }
        }
    }
    return pids;
}

fn kill_pids(pids: &Vec<String>) {
    for pid in pids {
        let task_kill = format!("taskkill /PID {pid} /F", pid = pid);
        let kill_command = Command::new("cmd.exe")
            .args(&["/c", task_kill.as_str()])
            .output();
        if kill_command.is_err() {
            // println!("failed to kill port");
        } else {
            println!("killed the port");
        }
    }
}

fn run_and_wait(config: Config) {
    if cfg!(target_os = "windows") {
        loop {
            let command = Command::new("cmd.exe")
                .current_dir(config.command_path.as_str())
                .args(&["/C", config.command.as_str()])
                .spawn();
            if command.is_err() {
                panic!(
                    "error while running init command {}",
                    command.err().unwrap()
                );
            };
            // here unwrap is fine
            if command.unwrap().wait().is_err() {
                panic!("error while waiting for init command");
            };
            println!("wait is done");
        }
    } else {
        loop {
            let split_command: Vec<&str> = config.command.split(" ").collect::<Vec<&str>>();
            let command = Command::new(split_command[0])
                .current_dir(config.command_path.as_str())
                .args(&split_command[1..])
                .spawn();
            if command.is_err() {
                panic!("error while running commmand {}", command.err().unwrap())
            }
            // wait for command
            if command.unwrap().wait().is_err() {
                panic!("error while waiting")
            }
            println!("wait is done")
        }
    }
}

fn kill(config: Config) {
    if cfg!(target_os = "windows") {
        let pids = get_pids(config.port.as_str());
        kill_pids(&pids);
    } else {
        // kill port on linux
        let command = format!(
            "kill -9 $(lsof -t -i:{} -sTCP:LISTEN)",
            config.port.as_str()
        );
        println!("command {}", command);
        let cmd = Command::new("bash")
            .args(&["-c", command.as_str()])
            .output();
        if cmd.is_err() {
            panic!("error while killing port {}", cmd.err().unwrap());
        }
    }
}

pub fn run_init_command(rx: Receiver<()>, config: Config) {
    let borrowed_config = config.clone();
    thread::spawn(move || {
        run_and_wait(borrowed_config);
    });
    loop {
        let _ = rx.recv().unwrap();
        kill(config.clone());
    }
}
