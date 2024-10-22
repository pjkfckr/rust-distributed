use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub task_type: TaskType,
    pub data: String,
    pub status: TaskStatus
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Processing,
    Completed,
    Failed
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskType {
    USER,
    BUSINESS
}