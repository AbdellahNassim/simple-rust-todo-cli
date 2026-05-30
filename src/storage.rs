use crate::todo::Todo;

use std::fs;

pub const DEFAULT_STORAGE_FILE: &str = "todos.json";

pub fn load_todos_from_file(file_path: &str) -> Vec<Todo> {
    let contents = fs::read_to_string(file_path);
    
    match contents {
        Ok(data) => {
            serde_json::from_str(&data).unwrap_or_else(|e| {
                eprintln!("Error parsing todos: {}", e);
                Vec::new()
            })
        }
        Err(e) => {
            eprintln!("Error loading todos: {}", e);
            Vec::new()
        }
    }

}

pub fn save_todos_to_file(todos: &Vec<Todo>, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {


    let json = serde_json::to_string_pretty(todos)?;

    fs::write(file_path, json)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::todo::Priority;
    use std::fs;
    
    #[test]
    fn test_save_and_load_todos() {
        let todos = vec![Todo::new(1, "Buy milk".to_string(), Priority::High)];
        let test_file = "test_todos.json";
        save_todos_to_file(&todos, test_file).unwrap();
        let loaded_todos = load_todos_from_file(test_file);
        assert_eq!(todos.len(), loaded_todos.len());
        assert_eq!(todos[0].get_id(), loaded_todos[0].get_id());
        assert_eq!(todos[0].get_title(), loaded_todos[0].get_title());
        assert_eq!(todos[0].get_priority(), loaded_todos[0].get_priority());
        fs::remove_file(test_file).unwrap();
    }
}