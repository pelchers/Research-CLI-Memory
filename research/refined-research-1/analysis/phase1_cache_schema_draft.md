# Phase 1 – Cache Schema Draft (Research)

## Objectives
- Produce a JSON schema outline for both long-term (`project_state.json`) and short-term session caches.
- Identify validation requirements, defaults, and versioning rules ahead of implementation (TASK-010/TASK-030).

## Proposed Schema (Draft)
```jsonc
{
  "version": 1,
  "metadata": {
    "created_at": "2025-10-18T00:00:00Z",
    "updated_at": "2025-10-18T00:00:00Z",
    "codex_cli_version": "1.x.y"
  },
  "git": {
    "branch": "main",
    "head": "abc123",
    "dirty": false,
    "remote": "origin"
  },
  "tasks": [
    {
      "id": "TASK-110",
      "title": "Short cache hook",
      "status": "in_progress",
      "adr": "ADR-110",
      "dependencies": ["TASK-030"],
      "milestone": "Phase 1",
      "evidence": [
        {"type": "commit", "ref": "abc123"},
        {"type": "analysis", "ref": "analysis/phase1_short_cache_research.md"}
      ],
      "last_updated": "2025-10-18T00:00:00Z"
    }
  ],
  "milestones": [
    {
      "id": "Phase-1",
      "title": "Short-Term Cache",
      "status": "in_progress",
      "summary": "Per-turn persistence groundwork."
    }
  ],
  "notes": [
    {
      "id": "N-001",
      "created_at": "2025-10-18T00:00:00Z",
      "content": "Observed drift warnings during test runs; reconcile before Phase 3."
    }
  ]
}
```

### Short-Term Session Cache Example
```jsonc
{
  "version": 1,
  "session_id": "rollout-2025-10-18T02-00-00-uuid",
  "branch": "main",
  "head": "def456",
  "entries": [
    {
      "turn_id": "5",
      "timestamp": "2025-10-18T02:05:30Z",
      "summary": "Assistant recommended cache hook insertion.",
      "actions": [
        {"type": "tool_call", "name": "apply_patch", "status": "approved"},
        {"type": "approval", "status": "approved"}
      ],
      "follow_up": [
        "Add schema tests",
        "Document cache write metrics"
      ]
    }
  ],
  "retention": {
    "max_entries": 50,
    "summaries_compacted": false
  }
}
```

## Validation Requirements
- Enforce schema version compatibility (reject unknown versions, or trigger migration).
- Require `git.branch` and `git.head`; allow `remote` and `dirty` to be optional.
- Task status limited to enum: `pending`, `in_progress`, `blocked`, `complete`.
- Evidence items must specify `type` and `ref`.
- Short cache retention should validate `max_entries > 0`.

## Migration Considerations
- Store schema version in both long-term and short-term files.
- Provide migration scripts to upgrade `version` on format changes (ADR-520).
- Keep backups before migration (align with ADR-030 cache manager requirements).

## Next Research Steps
- Model serde structs reflecting the JSON examples (Phase 0 implementation prototype).
- Define validation helpers for status enums and timestamp parsing.
- Outline unit tests for schema loading (e.g., invalid status, missing git info).

## References
- `sources/codex/codex-rs/core/src/rollout/recorder.rs`
- `analysis/phase1_short_cache_research.md`
- ADR-010, ADR-030 (pending implementation).
