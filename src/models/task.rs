use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Task{
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

impl Task{
    pub fn valid(self: &Self) -> bool{
        if self.completed || self.title.is_empty() { return false;}
        true
    }
}