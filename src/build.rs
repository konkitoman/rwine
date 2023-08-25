use std::{collections::HashMap, io::Read, os::fd::AsRawFd};

use crate::reactor::{command_task, Reactor, Task};

pub fn build() {
    let mut reactor = Reactor::default();

    let mut tasks = Vec::new();
    tasks.extend(build_dir("programs"));
    tasks.extend(build_dir("dlls"));

    for task in tasks {
        reactor.add_task(task);
    }

    reactor.run();

    return;
}

pub fn build_dir(dir: impl Into<std::path::PathBuf>) -> Vec<Task> {
    use std::fs::read_dir;
    let programs_dir = read_dir(dir.into()).expect("You should run this with `cargo run --`");
    let mut tasks = Vec::new();
    for program in programs_dir {
        let entry = program.unwrap();
        tasks.push(build_win_command(entry.path()));
    }
    tasks
}

pub fn build_win_command(path: std::path::PathBuf) -> Task {
    use std::process::Command;
    let name = path.file_name().unwrap().to_str().unwrap().to_owned();
    let mut command = Command::new("cargo");
    command
        .arg("build")
        .arg("--target=x86_64-pc-windows-msvc")
        .current_dir(&path);
    command_task(name, path.into_os_string().to_str().unwrap(), command).unwrap_or_else(
        |(name, what_is)| {
            println!("You need to have cargo installed");
            panic!("Cannot execute cargo build for: {name}, at: {what_is}")
        },
    )
}
