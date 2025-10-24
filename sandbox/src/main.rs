use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize)]
struct ProjectState {
    version: u32,
    tasks: Vec<TaskEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TaskEntry {
    id: String,
    title: String,
    status: TaskStatus,
    #[serde(default)]
    dependencies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
enum TaskStatus {
    Pending,
    InProgress,
    Blocked,
    Complete,
}

fn validate_tasks(tasks: &[TaskEntry]) -> Result<(), String> {
    let mut ids = HashSet::new();
    for task in tasks {
        if !ids.insert(task.id.clone()) {
            return Err(format!("duplicate task id: {}", task.id));
        }
    }
    for task in tasks {
        for dep in &task.dependencies {
            if !ids.contains(dep) {
                return Err(format!("unknown dependency '{}' in task {}", dep, task.id));
            }
        }
    }
    Ok(())
}

fn main() {
    let json = r#"{
        \"version\": 1,
        \"tasks\": [
            { \"id\": \"TASK-001\", \"title\": \"Bootstrap\", \"status\": \"pending\", \"dependencies\": [] },
            { \"id\": \"TASK-002\", \"title\": \"Prototype\", \"status\": \"in_progress\", \"dependencies\": [\"TASK-001\"] }
        ]
    }"#;

    match serde_json::from_str::<ProjectState>(json) {
        Ok(state) => match validate_tasks(&state.tasks) {
            Ok(()) => println!("Validation succeeded for {} tasks", state.tasks.len()),
            Err(err) => eprintln!("Validation error: {err}"),
        },
        Err(err) => eprintln!("Failed to parse project state: {err}"),
    }
}
