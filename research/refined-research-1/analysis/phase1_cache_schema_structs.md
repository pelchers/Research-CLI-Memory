# Phase 1 – Cache Schema Struct Prototypes (Research)

## Goals
- Translate the draft JSON schema into concrete Rust structures using `serde`.
- Identify validation helpers and error handling strategies for the cache manager prototype (TASK-030).
- Provide examples that can be copied into the Codex codebase during implementation.

## Long-Term Cache (Project State) Structs
```rust
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProjectState {
    pub version: u32,
    pub metadata: Metadata,
    pub git: GitSnapshot,
    pub tasks: Vec<TaskEntry>,
    pub milestones: Vec<MilestoneEntry>,
    #[serde(default)]
    pub notes: Vec<NoteEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub codex_cli_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitSnapshot {
    pub branch: String,
    pub head: String,
    #[serde(default)]
    pub dirty: bool,
    #[serde(default)]
    pub remote: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TaskEntry {
    pub id: String,
    pub title: String,
    #[serde(with = "task_status")]
    pub status: TaskStatus,
    pub adr: Option<String>,
    #[serde(default)]
    pub dependencies: Vec<String>,
    pub milestone: Option<String>,
    #[serde(default)]
    pub evidence: Vec<Evidence>,
    pub last_updated: OffsetDateTime,
}

#[derive(Debug, Clone)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Blocked,
    Complete,
}

mod task_status {
    use super::TaskStatus;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(status: &TaskStatus, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match status {
            TaskStatus::Pending => "pending",
            TaskStatus::InProgress => "in_progress",
            TaskStatus::Blocked => "blocked",
            TaskStatus::Complete => "complete",
        })
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<TaskStatus, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pending" => Ok(TaskStatus::Pending),
            "in_progress" => Ok(TaskStatus::InProgress),
            "blocked" => Ok(TaskStatus::Blocked),
            "complete" => Ok(TaskStatus::Complete),
            _ => Err(serde::de::Error::custom(format!("unknown status {value}"))),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Evidence {
    pub kind: EvidenceKind,
    pub reference: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceKind {
    Commit,
    Analysis,
    Document,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MilestoneEntry {
    pub id: String,
    pub title: String,
    #[serde(with = "task_status")]
    pub status: TaskStatus,
    #[serde(default)]
    pub summary: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteEntry {
    pub id: String,
    pub created_at: OffsetDateTime,
    pub content: String,
}
```

### Validation Checklist
- Verify `version == 1` (or use a migration path if higher).
- Ensure `tasks` and `milestones` do not have duplicate IDs.
- Confirm `dependencies` reference known task IDs.
- Enforce RFC3339 timestamps (handled by `OffsetDateTime` parsing).

## Short-Term Session Cache Structs
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionCache {
    pub version: u32,
    pub session_id: String,
    pub branch: String,
    pub head: String,
    #[serde(default)]
    pub entries: Vec<SessionEntry>,
    pub retention: RetentionPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionEntry {
    pub turn_id: String,
    pub timestamp: OffsetDateTime,
    pub summary: String,
    #[serde(default)]
    pub actions: Vec<ActionItem>,
    #[serde(default)]
    pub follow_up: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ActionItem {
    ToolCall { name: String, status: String },
    Approval { status: String },
    AutoCompact { status: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    #[serde(default = "default_max_entries")]
    pub max_entries: u32,
    #[serde(default)]
    pub summaries_compacted: bool,
}

fn default_max_entries() -> u32 {
    50
}
```

### Validation Checklist
- Reject cache if `max_entries == 0`.
- Promote `actions.status` into enums during implementation to avoid string parsing bugs.
- Enforce chronological ordering of `entries` (compare timestamps).

## Error Handling Strategy
- Use a dedicated error enum for schema validation (`CacheSchemaError`).
- Return descriptive messages (missing field, invalid enum, duplicate task ID).
- Integrate with cache manager to disable writes on repeated errors (align with ADR-510).

## Next Implementation Tasks
- Prototype `serde_json::from_str` + validation pipeline.
- Write unit tests for:
  - Invalid status strings.
  - Duplicate task IDs.
  - Session cache with zero `max_entries`.
- Prepare migration helpers that upgrade older schema versions.

## References
- `phase1_cache_schema_draft.md`
- `notes/working_notes.md`
- ADR-010 (schema), ADR-030 (cache manager), ADR-120 (retention).
