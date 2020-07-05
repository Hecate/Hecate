use crossbeam;
use postgres;
use std::thread;
use crate::{delta, mvt, webhooks};

#[derive(Debug,PartialEq)]
pub enum TaskType {
    Delta(i64),
    User(String),
    Style(i64),
    Meta
}

#[derive(Debug,PartialEq)]
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

#[derive(Clone)]
pub struct Worker {
    sender: crossbeam::Sender<Task>
}

impl Worker {
    pub fn new(database: String) -> Self {
        let (tx, rx) = crossbeam::channel::unbounded();

        thread::Builder::new().name(String::from("Hecate Daemon")).spawn(move || {
            async  {
                worker(rx, database).await;
            }
        }).unwrap();

        Worker {
            sender: tx
        }
    }

    pub fn queue(&self, task: Task) {
        if self.sender.send(task).is_err() {
            println!("WARN: Failed to write task to queue");
        }
    }
}

///
/// Main logic for web worker
///
async fn worker(rx: crossbeam::Receiver<Task>, database: String) {
    let mut conn = tokio_postgres::connect(format!("postgres://{}", database).as_str(), postgres::NoTls).await.unwrap();

    loop {
        let task = match rx.recv() {
            Ok(task) => task,
            Err(err) => {
                println!("Daemon: Failed to obtain task: {}", err);
                continue;
            }
        };

        if let Err(err) = webhooks::send(&mut conn.0, &task.job).await {
            println!("HecateError: {:?}", &err.to_string());
        }

        if let TaskType::Delta(delta_id) = task.job {
            let tiles = delta::tiles(&mut conn.0, delta_id, 14, 17).await.unwrap();

            if tiles.is_empty() {
                continue;
            }

            for tile in tiles {
                if mvt::regen(&mut conn.0, tile.2, tile.0 as u32, tile.1 as u32).await.is_some() {
                    println!("Daemon: Failed to generate tile: {:?}", tile);
                }
            }
        }
    }
}

