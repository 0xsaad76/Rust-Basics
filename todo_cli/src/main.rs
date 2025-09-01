use std::env;
use std::fs;
use std::io::{self, ErrorKind};

#[derive(serde::Serialize, serde::Deserialize)]
struct Task {
    description: String,
    completed: bool,
}

fn load_tasks() -> Result<Vec<Task>, io::Error> {
    match fs::read_to_string("tasks.json") {
        Ok(json) => {
            let tasks = serde_json::from_str(&json).unwrap_or_else(|_| vec![]);
            Ok(tasks)
        }
        Err(e) if e.kind() == ErrorKind::NotFound => {
            // If file is not found, it's not an error; just return an empty list
            Ok(vec![])
        }
        Err(e) => Err(e),
    }
}

fn save_tasks(tasks: &Vec<Task>) -> Result<(), io::Error> {
    // Convert the tasks vector to a JSON string
    let json = serde_json::to_string_pretty(tasks)?;
    // Write the JSON string to the file
    fs::write("tasks.json", json)
}

fn main() {
    let mut tasks = load_tasks().expect("Failed to load tasks from file.");

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: todo_cli [add|list]");
        return;
    }

    let command = &args[1];

    if command == "add" {
        if args.len() < 3 {
            println!("Usage: todo_cli add \"<task description>\"");
            return;
        }
        let new_task = Task {
            description: args[2].clone(),
            completed: false,
        };
        tasks.push(new_task);
        println!("Added task: '{}'", args[2]);
        // Saving the updatedd list to the file
        save_tasks(&tasks).expect("Failed to save tasks.");
    } else if command == "list" {
        println!("\n--- Your Tasks ---");
        for (index, task) in tasks.iter().enumerate() {
            let status = if task.completed { "[X]" } else { "[ ]" };
            println!("{} {} - {}", index + 1, status, task.description);
        }
        println!("------------------\n");
    } else {
        println!("INVALID COMMAND OPTIONO GIVEN! ");
    }
}
