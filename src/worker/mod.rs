use tilecover;
use crossbeam;
use std::thread;

pub enum TaskType {
    Delta(i64)
}

pub struct Task {
    job: TaskType
}

impl Task {
    pub fn new(tasktype: TaskType) -> Self {
        Task {
            job: tasktype
        }
    }
}

pub struct Worker {
    sender: crossbeam::Sender<Task>
}

impl Worker {
    pub fn new() -> Self {
        let (tx, rx) = crossbeam::channel::unbounded();

        thread::Builder::new().name(String::from("Hecate Daemon")).spawn(move || {
            worker(rx);
        }).unwrap();

        Worker {
            sender: tx,
        }
    }

    pub fn queue(&self, task: Task) {
        if self.sender.send(task).is_err() {
            format!("WARN: Failed to write task to queue");
        }
    }
}

///
/// Main logic for web worker
///
fn worker(rx: crossbeam::Receiver<Task>) {
    loop {
        let task = rx.recv().unwrap();

        match task.job {
            TaskType::Delta(delta_id) => {
                println!("{}", delta_id);
            }
        }
    }
}

