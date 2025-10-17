# Technical Requirements – Codex Memory Persistence Upgrade

## Architecture Overview
- **Cache Layer**
  - `project_state.json`: long-term artefact storing roadmap, task graph, git checkpoints, metadata.
  - `sessions/<timestamp>_<branch>.json`: short-lived per-session summaries.
- **Integration Points**
  - Hooks in `Session::spawn_task` / task completion to capture per-turn deltas.
  - Extension to `update_plan` handler to persist plan steps.
  - New reconciliation module comparing cached git info to live repo state.

## Data Model
```json
{
  "version": 1,
  "git": { "branch": "main", "head": "abc123", "dirty": false },
  "tasks": [
    { "id": "T-001", "title": "Implement cache writer", "status": "in_progress", "evidence": ["commit:abc123"] }
  ],
  "milestones": [],
  "notes": []
}
```
Short cache files reference the same IDs plus timestamps, approvals, and summarized outputs.

## Module Responsibilities
- **Cache Manager**: orchestrates loads/saves, conflict resolution, schema migrations.
- **Task Graph Service**: CRUD operations, CLI command handlers, plan-tool bridge.
- **Git Drift Checker**: compares cached head/branch vs `collect_git_info`, prompts user.
- **CLI Command Layer**: `codex cache show`, `codex tasks`, `codex cache reset`.

## Error Handling
- All write operations must fail gracefully with user-visible warnings; Codex execution continues.
- Cache corruption detection triggers automatic backup and reset with recovery instructions.

## Testing Strategy
- Unit tests for cache reads/writes, drift detection, plan integration.
- Integration tests across branch switches, rebases, detached HEAD scenarios.
- Performance benchmarks to ensure turn latency stays within target.

## Deployment Considerations
- Feature flag `memory.cache.enabled` defaults to `false`.
- Provide migration script reading historical rollouts to seed initial cache.
- Document manual edit workflows and git conflict resolution steps.
