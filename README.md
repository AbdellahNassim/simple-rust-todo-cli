# todo-cli

A simple command-line todo manager written in Rust. Add tasks with priorities, list them with filters, mark them complete, and delete them. Todos are persisted to a local JSON file.

## Features

- **Add** todos with a title and priority (`low`, `medium`, `high`)
- **List** todos with optional filters (all, completed, incomplete, or by priority)
- **Mark done** by ID
- **Delete** by ID
- **Colored terminal output** for status and priority
- **JSON persistence** in `todos.json` (created/updated in the current working directory)

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70+ recommended; project uses edition 2024)

## Getting started

Clone the repository and build from the project root:

```bash
git clone https://github.com/AbdellahNassim/simple-rust-todo-cli
cd todo-cli
cargo build --release
```

Run with Cargo (development):

```bash
cargo run -- <command> [options]
```

Or run the release binary directly:

```bash
# Windows
.\target\release\todo-cli.exe <command> [options]

# Linux / macOS
./target/release/todo-cli <command> [options]
```

> **Important:** Run commands from the project directory (or any directory where you want `todos.json` to live). The app reads and writes `todos.json` relative to your **current working directory**.

## Usage

### Add a todo

Priority is required. Use `-p` or `--priority`:

```bash
cargo run -- add "Buy milk" -p high
cargo run -- add "Walk the dog" --priority low
cargo run -- add "Read a chapter" -p medium
```

**Accepted priority values** (case-insensitive):

| Priority | Aliases |
|----------|---------|
| `low`    | `l`     |
| `medium` | `med`, `m` |
| `high`   | `h`     |

Example output:

```text
Added task: Buy milk with ID: 1 and priority: High
```

### List todos

Default filter shows all todos:

```bash
cargo run -- list
```

Filter with `-f` or `--filter`:

```bash
cargo run -- list -f all          # all todos (default)
cargo run -- list -f completed    # only completed
cargo run -- list -f incomplete   # only not completed
cargo run -- list -f high         # only high priority
cargo run -- list -f medium
cargo run -- list -f low
```

Example output:

```text
Todos:
[❌] 1: Buy milk - High
[✅] 2: Walk the dog - Low
```

### Mark a todo as done

```bash
cargo run -- done 1
```

### Delete a todo

```bash
cargo run -- delete 2
```

### Help

```bash
cargo run -- --help
cargo run -- add --help
cargo run -- list --help
```

## Running Tests
To run the tests for this project, use the following command:

```bash
cargo test
```

This will execute all unit and integration tests defined in the `src/` and `tests/` directories.


## Command reference

| Command | Description |
|---------|-------------|
| `add <TASK> -p <PRIORITY>` | Create a new todo |
| `list [-f FILTER]` | Show todos (default filter: `all`) |
| `done <ID>` | Mark todo as completed |
| `delete <ID>` | Remove todo by ID |

**List filters:** `all`, `completed`, `incomplete`, `high`, `medium`, `low`

## Data storage

Todos are stored in `todos.json` as pretty-printed JSON:

```json
[
  {
    "id": 1,
    "title": "Buy milk",
    "completed": false,
    "priority": "High"
  }
]
```

IDs are assigned automatically (max existing ID + 1). New todos start with `completed: false`.

## Project structure

```text
todo-cli/
├── Cargo.toml
├── todos.json          # created/updated at runtime (in cwd)
└── src/
    ├── main.rs         # CLI entry point (clap)
    ├── todo.rs         # Todo model and Priority enum
    ├── service.rs      # Business logic (add, list, done, delete)
    └── storage.rs      # Load/save todos.json
```

## Dependencies

- [clap](https://github.com/clap-rs/clap) — CLI parsing
- [serde](https://serde.rs/) / [serde_json](https://github.com/serde-rs/json) — JSON serialization
- [colored](https://github.com/colored-rs/colored) — terminal colors

## License

This project is for learning purposes. You can use it as you please.
