mod todo;
mod storage;
mod service;

use clap::{Parser, Subcommand};
use service::{add_todo, list_todos, mark_todo_as_completed, delete_todo};
use todo::Priority;
#[derive(Parser)]
#[command(name = "todo")]
#[command(version = "1.0.0")]
#[command(about = "A simple CLI for managing your todo list")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

}

#[derive(Subcommand)]
enum Commands {
    Add {
        task: String,
        #[arg(short, long, value_parser = clap::value_parser!(Priority))]
        priority: Priority,
    },
    List,
    Done {
        id: u32,
    },
    Delete {
        id: u32,
    },
}

fn main() {
    
    let cli = Cli::parse();
    match cli.command {
        Commands::Add { task, priority } => {
            add_todo(task, priority);
        }
        Commands::List => {
            list_todos();
        },
        Commands::Done { id } => {
            mark_todo_as_completed(id);
        },
        Commands::Delete { id } => {
            delete_todo(id);
        },
    }

}
