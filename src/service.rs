use crate::storage::{load_todos, save_todos};
use crate::todo::{Todo, Priority};
use colored::*;

fn get_next_id(todos: &Vec<Todo>) -> u32 {
    let max_id = todos.iter().map(|t| t.get_id()).max().unwrap_or(0);
    max_id + 1
}

pub fn add_todo(title: String, priority: Priority) {
    let mut todos = load_todos();
    let new_id = get_next_id(&todos);
    let todo = Todo::new(new_id, title.clone(), priority.clone());
    todos.push(todo);
    if let Err(e) = save_todos(&todos) {
        eprintln!("Error saving todos: {}", e);
    }
    let priority_str = match priority {
        Priority::Low => "Low".green(),
        Priority::Medium => "Medium".yellow(),
        Priority::High => "High".red(),
    };
    println!("Added task: {} with ID: {} and priority: {}", title, new_id, priority_str);
}

pub fn list_todos(filter: String) {
    let todos = load_todos();
    if todos.is_empty() {
        println!("{}", "No todos found".yellow());
    } else {
        println!("{}", "Todos:".green());

        for todo in todos.iter().filter(|t| match filter.as_str() {
            "all" => true,
            "completed" => t.is_completed(),
            "incomplete" => !t.is_completed(),
            "high" => t.get_priority() == &Priority::High,
            "medium" => t.get_priority() == &Priority::Medium,
            "low" => t.get_priority() == &Priority::Low,
            _ => false,
        }) {
            let status = if todo.is_completed() { "✅" } else { "❌" };
            let priority_str = match todo.get_priority() {
                Priority::Low => "Low".green(),
                Priority::Medium => "Medium".yellow(),
                Priority::High => "High".red(),
            };
            let text = if todo.is_completed() {
                todo.get_title().dimmed()
            } else {
                todo.get_title().normal()
            };
            println!("[{}] {}: {} - {}", status, todo.get_id().to_string().cyan().bold(), text.to_string(), priority_str.to_string());
        }
    }
}

pub fn mark_todo_as_completed(id: u32) {
    let mut todos = load_todos();
    if let Some(todo) = todos.iter_mut().find(|t| t.get_id() == id) {
        todo.set_completed(true);
        if let Err(e) = save_todos(&todos) {
            eprintln!("Error saving todos: {}", e);
        }
        println!("Marked todo with ID: {} as completed", id);
    } else {
        println!("Todo with ID: {} not found", id);
    }
}

pub fn delete_todo(id: u32) {
    let mut todos = load_todos();
    let initial_len = todos.len();
    todos.retain(|t| t.get_id() != id);
    if todos.len() < initial_len {
        if let Err(e) = save_todos(&todos) {
            eprintln!("Error saving todos: {}", e);
        }
        println!("Deleted todo with ID: {}", id);
    } else {
        println!("Todo with ID: {} not found", id);
    }
}
