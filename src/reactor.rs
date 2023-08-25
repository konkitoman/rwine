use std::{
    collections::{HashMap, HashSet},
    io::Read,
    os::fd::AsRawFd,
    thread::JoinHandle,
};

use colored::Colorize;
use polling::Poller;

pub trait Stdi: Read + AsRawFd {}

impl<T: Read + AsRawFd> Stdi for T {}

pub struct InitResult {
    pub stdout: Box<dyn Stdi>,
    pub stderr: Box<dyn Stdi>,
    pub task: JoinHandle<i32>,
}

#[derive(Default)]
pub struct Task {
    pub id: usize,
    pub depends: HashSet<usize>,
    pub name: String,
    pub what_is: String,
    pub stdout: Option<Box<dyn Stdi>>,
    pub stderr: Option<Box<dyn Stdi>>,
    pub init: Option<Box<dyn FnOnce() -> InitResult>>,
    pub task: Option<JoinHandle<i32>>,
}

impl Task {
    pub fn new<F: FnOnce() -> InitResult + 'static>(
        name: impl Into<String>,
        what_is: impl Into<String>,
        init: F,
    ) -> Self {
        Self {
            name: name.into(),
            what_is: what_is.into(),
            init: Some(Box::new(init)),
            ..Default::default()
        }
    }

    pub fn depend(mut self, id: usize) -> Self {
        self.depends.insert(id);
        self
    }
}

#[derive(Default)]
pub struct Reactor {
    couter: usize,
    tasks: HashMap<usize, Task>,
    done: Vec<usize>,
    failed: Vec<usize>,
    cannot_complete: HashMap<usize, HashSet<usize>>,
}

impl Reactor {
    pub fn add_task(&mut self, mut task: Task) -> usize {
        let id = self.couter;
        task.id = id;
        self.tasks.insert(id, task);
        self.couter += 1;
        id
    }

    pub fn run(&mut self) {
        let poller = Poller::new().unwrap();

        {
            let len = self.tasks.len();
            for task in self.tasks.values_mut() {
                for depend in task.depends.iter() {
                    if *depend >= len {
                        eprintln!("Invalid depend {depend} for : {}", task.id);
                    }
                }
                Self::try_start_task(task, &[], &[], &mut self.cannot_complete, &poller);
            }
        }

        let mut events = Vec::<polling::Event>::default();
        let mut buffer = [0; 4096];

        while self.tasks.len() != self.done.len() + self.failed.len() + self.cannot_complete.len() {
            poller.wait(&mut events, None).unwrap();

            let mut try_init_all = false;

            for event in events.drain(..) {
                let key = event.key / 2;
                let is_stdout = event.key % 2 == 0;

                let tasks = self.tasks.len()
                    - (self.done.len() + self.failed.len() + self.cannot_complete.len());
                let task = self.tasks.get_mut(&key).unwrap();

                if is_stdout {
                    if let Some(stdout) = &task.stdout {
                        poller.modify(stdout.as_raw_fd(), event).unwrap();
                    }
                } else if let Some(stderr) = &task.stderr {
                    poller.modify(stderr.as_raw_fd(), event).unwrap();
                }

                // Print content

                if is_stdout {
                    if let Some(stdout) = &mut task.stdout {
                        let mut tmp_buffer = Vec::<u8>::new();
                        if let Ok(len) = stdout.read(&mut buffer) {
                            tmp_buffer.extend(&buffer[..len]);
                        }
                        let text = String::from_utf8_lossy(&tmp_buffer).to_string();

                        if !text.is_empty() {
                            let lines = text.split('\n').collect::<Vec<&str>>();

                            for line in lines {
                                println!("[{tasks}] {}: {}", task.name.green(), line)
                            }
                        }
                    }
                } else if let Some(stderr) = &mut task.stderr {
                    let mut tmp_buffer = Vec::<u8>::new();
                    if let Ok(len) = stderr.read(&mut buffer) {
                        tmp_buffer.extend(&buffer[..len]);
                    }
                    let text = String::from_utf8_lossy(&tmp_buffer).to_string();

                    if !text.is_empty() {
                        let lines = text.split('\n').collect::<Vec<&str>>();

                        for line in lines {
                            eprintln!("[{tasks}] {}: {}", task.name.red(), line)
                        }
                    }
                }

                // See if the task ended

                if let Some(task_thread) = task.task.take() {
                    if task_thread.is_finished() {
                        if let Ok(result) = task_thread.join() {
                            if result == 0 {
                                log::info!("Task compilted: {}", task.name);
                                self.done.push(task.id);
                            } else {
                                log::error!("Task failed: {}, with: {result}", task.name);
                                self.failed.push(task.id);
                            }
                        } else {
                            self.failed.push(task.id);
                        }
                        task.stdout.take();
                        task.stderr.take();
                        try_init_all = true;
                    } else {
                        task.task = Some(task_thread);
                    }
                }
            }

            if try_init_all {
                for task in self.tasks.values_mut() {
                    Self::try_start_task(
                        task,
                        &self.done,
                        &self.failed,
                        &mut self.cannot_complete,
                        &poller,
                    );
                }
            }
        }

        if !self.done.is_empty() {
            let text = "Sucesses: ".green();
            println!("{text}");
            for done in self.done.iter() {
                let task = self.tasks.get(done).unwrap();
                println!("    {}: {}", task.name.green(), task.what_is.bright_black());
            }
        }

        if !self.failed.is_empty() {
            let text = "Fail's".red();
            println!("{text}");
            for fail in self.failed.iter() {
                let task = self.tasks.get(fail).unwrap();
                println!("    {}: {}", task.name.red(), task.what_is.bright_black());
            }
        }

        if !self.cannot_complete.is_empty() {
            let text = "Cannot complete: ".white();
            println!("{text}");
            for (id, because) in self.cannot_complete.iter() {
                let task = self.tasks.get(id).unwrap();
                println!(
                    "    {}: {}: ",
                    task.name.yellow(),
                    task.what_is.bright_black()
                );
                for because in because {
                    let task = self.tasks.get(because).unwrap();
                    println!(
                        "        {}: {}",
                        task.name.red(),
                        task.what_is.bright_black()
                    );
                }
            }
        }
    }

    pub fn try_start_task(
        task: &mut Task,
        done: &[usize],
        failed: &[usize],
        cannot_complete: &mut HashMap<usize, HashSet<usize>>,
        poller: &Poller,
    ) {
        let mut done_counter = 0;
        for depend in task.depends.iter() {
            if done.contains(depend) {
                done_counter += 1;
            }
            if failed.contains(depend) {
                cannot_complete.entry(*depend).or_default().insert(*depend);
            }
        }

        let should_start = done_counter == task.depends.len();

        if !should_start {
            return;
        }

        let Some(init) = task.init.take() else { return };
        let result = init();
        let stdout = result.stdout;
        let stderr = result.stderr;

        task.task = Some(result.task);

        poller
            .add(
                stdout.as_raw_fd(),
                polling::Event {
                    key: task.id * 2,
                    readable: true,
                    writable: false,
                },
            )
            .unwrap();

        poller
            .add(
                stderr.as_raw_fd(),
                polling::Event {
                    key: (task.id * 2) + 1,
                    readable: true,
                    writable: false,
                },
            )
            .unwrap();

        task.stdout = Some(stdout);
        task.stderr = Some(stderr);

        log::info!("Task Started: {}", task.name);
    }
}

pub fn command_task(
    name: impl Into<String>,
    what_is: impl Into<String>,
    mut command: std::process::Command,
) -> Result<Task, (String, String)> {
    use std::process::Stdio;
    command
        .stdin(Stdio::inherit())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Ok(mut child) = command.spawn() {
        Ok(Task::new(name, what_is, move || {
            crate::reactor::InitResult {
                stdout: Box::new(child.stdout.take().unwrap()),
                stderr: Box::new(child.stderr.take().unwrap()),
                task: std::thread::spawn(move || {
                    child
                        .wait()
                        .expect("Failed to run!")
                        .code()
                        .unwrap_or(i32::MAX)
                }),
            }
        }))
    } else {
        Err((name.into(), what_is.into()))
    }
}
