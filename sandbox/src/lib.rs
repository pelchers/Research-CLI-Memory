use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectState {
    pub version: u32,
    pub tasks: Vec<TaskEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskEntry {
    pub id: String,
    pub title: String,
    pub status: TaskStatus,
    #[serde(default)]
    pub dependencies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Pending,
    InProgress,
    Blocked,
    Complete,
}

pub fn validate_tasks(tasks: &[TaskEntry]) -> Result<(), String> {
    let mut ids = HashSet::new();
    for task in tasks {
        if !ids.insert(task.id.clone()) {
            return Err(format!("duplicate task id: {}", task.id));
        }
    }
    for task in tasks {
        for dep in &task.dependencies {
            if !ids.contains(dep) {
                return Err(format!("unknown dependency '{dep}' in task {}", task.id));
            }
        }
    }
    Ok(())
}
