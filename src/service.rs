use crate::storage::{load_todos, save_todos};
use crate::todo::Todo;

fn get_next_id(todos: &Vec<Todo>) -> u32 {
    let max_id = todos.iter().map(|t| t.get_id()).max().unwrap_or(0);
    max_id + 1
}

pub fn add_todo(title: String) {
    let mut todos = load_todos();
    let new_id = get_next_id(&todos);
    let todo = Todo::new(new_id, title.clone());
    todos.push(todo);
    if let Err(e) = save_todos(&todos) {
        eprintln!("Error saving todos: {}", e);
    }
    println!("Added task: {} with ID: {}", title, new_id);
}

pub fn list_todos() {
    let todos = load_todos();
    if todos.is_empty() {
        println!("No todos found");
    } else {
        println!("Todos:");
        for todo in todos {
            let status = if todo.is_completed() { "✅" } else { "❌" };
            println!("[{}] {}: {}", status, todo.get_id(), todo.get_title());
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