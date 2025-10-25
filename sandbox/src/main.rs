use cache_validation::{validate_tasks, ProjectState};

fn main() {
    let json = include_str!("../fixtures/valid_project.json");
    match serde_json::from_str::<ProjectState>(json) {
        Ok(state) => match validate_tasks(&state.tasks) {
            Ok(()) => println!("Validation succeeded for {} tasks", state.tasks.len()),
            Err(err) => eprintln!("Validation error: {err}"),
        },
        Err(err) => eprintln!("Failed to parse project state: {err}"),
    }
}
