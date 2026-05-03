use crate::models::task::{Task};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ResponseNewTask{
    pub status_code: u32,
    pub task: Task
}
#[derive(Deserialize, Serialize)]
pub struct ResponseErro{
    pub status_code: u32,
    pub message: String
}