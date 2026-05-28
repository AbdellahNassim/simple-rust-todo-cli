use crate::todo::Todo;

use std::fs;

const STORAGE_FILE: &str = "todos.json";

pub fn load_todos() -> Vec<Todo> {
    let contents = fs::read_to_string(STORAGE_FILE);
    
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

pub fn save_todos(todos: &Vec<Todo>) -> Result<(), Box<dyn std::error::Error>> {


    let json = serde_json::to_string_pretty(todos)?;

    fs::write(STORAGE_FILE, json)?;

    Ok(())
}