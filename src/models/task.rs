use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Task{
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct NewTask{
    pub title: String,
    pub completed: bool,
}

impl NewTask{
    pub fn valid(self: &Self) -> bool{
        !self.title.trim().is_empty()
    }
}