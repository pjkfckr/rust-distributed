use crate::models::task::{Task, TaskStatus};

pub async fn process_task(mut task: Task) {
    println!("Processing task: {:?}", task);

    task.status = TaskStatus::Completed;
}