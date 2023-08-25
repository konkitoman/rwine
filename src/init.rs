use crate::reactor::{command_task, Reactor};

pub fn init() {
    use std::process::Command;

    let mut reactor = Reactor::default();

    let mut command = Command::new("cargo");
    command.arg("install").arg("xwin");
    let task = command_task("install xwin", "installing xwin", command).unwrap();

    let install_task = reactor.add_task(task);

    let mut command = Command::new("xwin");
    command.arg("splat");
    let task = command_task(
        "setup windows libs",
        "running xwin to install windows sdk localy",
        command,
    )
    .unwrap()
    .depend(install_task);

    reactor.add_task(task);

    reactor.run();
}
