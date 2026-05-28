use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone )]
pub struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

impl Todo {
    pub fn new(id: u32, title: String) -> Self {
        Self { id, title, completed: false }
    }
    pub fn is_completed(&self) -> bool {
        self.completed
    }
    pub fn get_id(&self) -> u32 {
        self.id
    }
    pub fn get_title(&self) -> &str {
        &self.title
    }
    pub fn set_completed(&mut self, completed: bool) {
        self.completed = completed;
    }
}