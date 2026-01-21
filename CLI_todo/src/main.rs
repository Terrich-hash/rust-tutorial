use clap::{Parser, Subcommand};
use std::fs::{self, OpenOptions};
use std::io::Write;

const FILE_NAME: &str = "todo.txt";

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "Simple CLI Todo App")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        task: String,
    },
    List,
    Remove {
        index: usize,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { task } => add_task(&task),
        Commands::List => list_tasks(),
        Commands::Remove { index } => remove_task(index),
    }
}

fn add_task(task: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(FILE_NAME)
        .expect("Cannot open todo file");

    writeln!(file, "{task}").expect("Cannot write task");
    println!("✅ Added: {task}");
}

fn list_tasks() {
    let content = fs::read_to_string(FILE_NAME).unwrap_or_default();

    if content.is_empty() {
        println!("📭 No tasks found");
        return;
    }

    for (i, task) in content.lines().enumerate() {
        println!("{}: {}", i + 1, task);
    }
}

fn remove_task(index: usize) {
    let content = fs::read_to_string(FILE_NAME).unwrap_or_default();
    let mut tasks: Vec<&str> = content.lines().collect();

    if index == 0 || index > tasks.len() {
        println!("❌ Invalid task number");
        return;
    }

    let removed = tasks.remove(index - 1);
    fs::write(FILE_NAME, tasks.join("\n")).expect("Cannot update file");

    println!("🗑 Removed: {removed}");
}
