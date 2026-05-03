use crate::models::task::Task;

pub struct AppState{
    pub tasks: Vec<Task>,
    pub next_id: u32
}