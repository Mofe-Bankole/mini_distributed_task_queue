use std::time::Duration;

#[derive(Debug)]
pub struct Task{
    pub id : i32,
    pub task : String,
    pub duration : Duration,
}


// Created a sender and receiver using flume 
// Solved the issue of MPMC with flume
pub type TaskQueueSender = flume::Sender<Task>;
pub type TaskQueueReceiver = flume::Receiver<Task>;

pub fn create_new_task() -> (TaskQueueSender, TaskQueueReceiver) {
    flume::unbounded()
}