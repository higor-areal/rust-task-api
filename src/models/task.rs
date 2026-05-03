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
        if self.title.is_empty() { return false;}
        true
    }
}