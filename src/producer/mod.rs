use crate::task::Task;
use flume::Sender;
use rand::Rng;
use std::time::Duration;

pub fn generate_id() -> i32 {
    let mut range = rand::rng();
    range.random_range(1..151)
}

pub fn producer(sender: Sender<Task>) {
    for i in 1..151 {
        let task = Task {
            id: generate_id(),
            task: format!("Do work {}", i),
            duration: Duration::from_secs(6),
        };

        sender.send(task).unwrap()
    }
}
