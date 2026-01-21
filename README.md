🦀 Rust Learning Repository

A hands-on Rust learning repository containing multiple small projects built to understand Rust fundamentals, Cargo workflows, CLI development, and safe systems programming.

This repository focuses on learning by building, not just reading.

📂 Repository Structure
```
rust-learn/
├── hello_world/
│   └── src/main.rs
├── guessing_game/
│   ├── Cargo.toml
│   └── src/main.rs
├── cli_todo/
│   ├── Cargo.toml
│   └── src/main.rs
└── Cargo.toml   # Workspace configuration
```
🚀 Projects Included
🟢 Hello World
A minimal Rust program to verify the Rust toolchain and understand the basic program structure.

**Concepts covered**
- `main` function – Entry point of every Rust program
- `println!` macro – Prints output to the console
- Compiling and running Rust code – Using `cargo run` and `rustc`

Run
```
cd hello_world
cargo run
```
🎯 Guessing Game
A terminal-based number guessing game inspired by the official Rust Book.

**Features**
- Random number generation (rand)
- User input handling
- Pattern matching (match)
- Error handling with Result
- Loop control and comparison logic

Run
```
cd guessing_game
cargo run
```
✅ CLI Todo App
A command-line todo list application built using idiomatic Rust.

**Features**
- Add, list, and remove tasks
- Persistent storage using a local file
- Clean CLI interface with clap
- Enum-based command handling

Example usage
```
cd cli_todo
cargo run -- add "Learn Rust"
cargo run -- list
cargo run -- remove 1
```
🧠 Concepts Practiced
- Rust ownership and borrowing basics
- Cargo project structure
- Workspaces vs crates
- External dependencies (`rand`, `clap`)
- File I/O
- Enums and pattern matching
- Error handling without panics
- Writing real-world CLI tools

🛠 Requirements
- Rust (latest stable)
- Cargo (bundled with Rust)

Install Rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

📦 Cargo Workspace
This repository uses a Cargo workspace for clean multi-project management.

Root `Cargo.toml`
```
[workspace]
members = [
    "hello_world",
    "guessing_game",
    "cli_todo"
]
```
Each project is an independent crate with its own `Cargo.toml`

🎯 Purpose
- Learn Rust through real, runnable projects
- Build a strong foundation in systems programming
- Understand Cargo, dependencies, and workspaces
- Practice writing clean, idiomatic Rust code

🚧 Planned Improvements
- Add unit and integration tests
- Improve error handling
- Convert projects into reusable libraries
- Add async Rust examples
- Add CI with GitHub Actions

📜 License
This project is licensed under the **MIT License**.  
Free to use, modify, and learn from.

⭐ Tip
- If you’re learning Rust, clone this repo and build everything yourself.
- Breaking and fixing things is how Rust really clicks.
