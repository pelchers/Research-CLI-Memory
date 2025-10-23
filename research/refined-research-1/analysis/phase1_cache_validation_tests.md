# Phase 1 – Validation & Test Plan

## Goals
- Define helper functions to validate schema structs.
- Outline unit/integration test strategy for cache manager prototype.

## Validation Helpers
- alidate_task_entries(&ProjectState)
  - Check for duplicate task IDs.
  - Ensure dependencies reference existing IDs.
  - Assert status transitions align with ADR statuses.
- alidate_git_snapshot(&GitSnapshot)
  - Branch non-empty, head is hex SHA.
- alidate_session_entries(&SessionCache)
  - Entries sorted by timestamp.
  - etention.max_entries > 0.
  - Action enums parsed from strings.

## Test Strategy
- Unit tests for serde parse success/failure.
- Fixtures: alid_project_state.json, invalid_status.json, etc.
- Integration: simulate cache writes + load/validate cycle.

## TODO
- Create fixture samples.
- Draft test functions referencing serde structs.
- Link to ADR-010/030 when ready.

