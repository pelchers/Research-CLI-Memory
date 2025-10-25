use cache_validation::{validate_tasks, ProjectState};

fn load_fixture(name: &str) -> ProjectState {
    let path = format!("../fixtures/{name}");
    let data = std::fs::read_to_string(path).expect("fixture should exist");
    serde_json::from_str(&data).expect("fixture should parse")
}

#[test]
fn accepts_valid_project_state() {
    let state = load_fixture("valid_project.json");
    assert!(validate_tasks(&state.tasks).is_ok());
}

#[test]
fn rejects_duplicate_ids() {
    let state = load_fixture("invalid_duplicate.json");
    let err = validate_tasks(&state.tasks).expect_err("expected duplicate error");
    assert!(err.contains("duplicate task id"));
}

#[test]
fn rejects_missing_dependencies() {
    let state = load_fixture("invalid_dependency.json");
    let err = validate_tasks(&state.tasks).expect_err("expected dependency error");
    assert!(err.contains("unknown dependency"));
}
