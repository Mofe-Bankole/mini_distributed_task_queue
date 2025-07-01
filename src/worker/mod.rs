use crate::task::Task;
use flume::Receiver;
#[allow(unused)]
use std::{sync::Arc, thread, time::Duration};

#[derive(Debug)]
pub struct Worker {
    pub id: usize,
}

impl Worker {
    pub async fn new(id: usize) -> Worker {
        Self { id }
    }

    pub async fn consume_task(&self, receiver: Receiver<Task>) {
        while let Ok(task) = receiver.recv_async().await {
            println!("Worker {:?} processing task {:?}", self.id, task.id);
            thread::sleep(task.duration);
            println!(
                "Worker {:?} has successfully completed task {:?}",
                self.id, task.id
            )
        }
    }
}
