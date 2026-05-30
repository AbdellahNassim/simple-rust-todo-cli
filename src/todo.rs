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

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "low" | "l" => Ok(Priority::Low),
            "medium" | "med" | "m" => Ok(Priority::Medium),
            "high" | "h" => Ok(Priority::High),
            _ => Err(format!("invalid priority '{s}'. Expected: low, medium, or high")),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_parsing() {
        let priority = Priority::from_str("high").unwrap();
        assert!(matches!(priority, Priority::High));

        let priority = Priority::from_str("medium").unwrap();
        assert!(matches!(priority, Priority::Medium));

        let priority = Priority::from_str("low").unwrap();
        assert!(matches!(priority, Priority::Low));

        let priority = Priority::from_str("invalid").unwrap_err();
    }

    #[test]
    fn test_todo_new() {
        let todo = Todo::new(1, "Buy milk".to_string(), Priority::High);
        assert_eq!(todo.get_id(), 1);
        assert_eq!(todo.get_title(), "Buy milk");
        assert!(matches!(todo.get_priority(), &Priority::High));
    }

    #[test]
    fn test_todo_is_completed() {
        let mut todo = Todo::new(1, "Buy milk".to_string(), Priority::High);
        assert!(!todo.is_completed());
        todo.set_completed(true);
        assert!(todo.is_completed());
    }
    
    
}