use std::str::FromStr;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Priority {
    Low,
    Medium,
    High,
}
impl Priority {
    pub fn to_string(&self) -> &str {
        match self {
            Priority::Low => "Low",
            Priority::Medium => "Medium",
            Priority::High => "High",
        }
    }
}

impl FromStr for Priority {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "low" | "l" => Ok(Priority::Low),
            "medium" | "med" | "m" => Ok(Priority::Medium),
            "high" | "h" => Ok(Priority::High),
            _ => Err(format!(
                "invalid priority '{s}'. Expected: low, medium, or high"
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone )]
pub struct Todo {
    id: u32,
    title: String,
    completed: bool,
    priority: Priority,
}

impl Todo {
    pub fn new(id: u32, title: String, priority: Priority) -> Self {
        Self { id, title, completed: false, priority }
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
    pub fn get_priority(&self) -> &Priority {
        &self.priority
    }
    pub fn set_completed(&mut self, completed: bool) {
        self.completed = completed;
    }
}