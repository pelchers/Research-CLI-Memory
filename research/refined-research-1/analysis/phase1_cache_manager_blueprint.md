# Phase 1 – Cache Manager Blueprint (Research)

## Goals
- Outline the responsibilities and API surface of the new `CacheManager`.
- Describe how schema loading, validation, and backups interact.
- Provide pseudocode ready for translation into Codex once the feature flag is enabled.

## Key Responsibilities
1. **Discovery**
   - Resolve cache directory: `~/.codex/cache/{project_id}/`.
   - Ensure directories exist (create as needed).
2. **Loading**
   - Read `project_state.json`.
   - Deserialize into `ProjectState` structs (see `phase1_cache_schema_structs.md`).
   - Run validation helpers; return typed errors on failure.
3. **Writing**
   - Serialize updated state to JSON (`serde_json::to_string_pretty`).
   - Write to temp file + `fs::rename` for atomic commit.
   - Create timestamped backup before overwriting (e.g., `backups/project_state-YYYYMMDDHHMMSS.json`).
4. **Short Cache Management**
   - Maintain session-specific cache files under `short_cache/{branch}/`.
   - Prune according to retention policy (ADR-120).
5. **Error Handling**
   - On any failure, log warning and disable cache for current session (shadow mode fallback).
   - Expose errors to telemetry layer (ADR-430).

## Proposed API Surface
```rust
pub struct CacheManager {
    root: PathBuf,
    project_state: Option<ProjectState>,
}

impl CacheManager {
    pub async fn load(config: &Config) -> Result<Self, CacheError>;
    pub async fn project_state(&self) -> Option<&ProjectState>;
    pub async fn update_project_state<F>(&mut self, updater: F) -> Result<(), CacheError>
    where
        F: FnOnce(&mut ProjectState);
    pub async fn append_short_cache(&self, entry: SessionEntry) -> Result<(), CacheError>;
    pub async fn disable_for_session(&self);
}
```

## CacheError Enumeration (Draft)
```rust
pub enum CacheError {
    Io(std::io::Error),
    Deserialize(serde_json::Error),
    Validation(String),
    DisabledForSession,
}
```

## Integration Notes
- `CacheManager::load` should run during session boot if the feature flag is enabled.
- Session hook (see `phase1_cache_hook_plan.md`) calls `append_short_cache`.
- High-level components (e.g., `Session`) observe `CacheError::DisabledForSession` and skip further writes.
- A background task can periodically persist `ProjectState` (e.g., on turn completion or idle).

## Next Steps
- Prototype `CacheManager::load` and validation flow in a sandbox.
- Create unit tests for backup rotation and retention pruning.
- Document configuration toggles in `.codex/docs/project/implementation-plan.md` when implementation begins.
