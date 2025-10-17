# ADR-010 Cache Schema Definition

- **Status:** Planning
- **Task:** TASK-010
- **Owner:** Codex Memory Research Team
- **Last Updated:** 2025-10-16

## Context
Codex currently persists session rollouts and global history without a canonical project-state representation (`research/refined-research-1/analysis/codex_gap_analysis.md:3`). Before layering new persistence features, we need an explicit, versioned schema for both long-term (`project_state.json`) and short-term (`sessions/<timestamp>_<branch>.json`) cache artefacts (`research/refined-research-1/analysis/persistence_recommendations.md:21`).

## Decision
Create a JSON-based schema (Version 1) that scopes:
- Global metadata: schema version, creation timestamps, Codex CLI version.
- Git snapshot: branch, commit hash, dirty flag, optional remote.
- Task graph: nodes, edges, status, evidence links.
- Milestones/backlog, notes, and telemetry counters.
- Short-cache entries: turn id, timestamp, summary, approvals, diff references.

The schema will be described in prose + JSON examples and validated via a lightweight Rust struct with serde validation.

## Implementation Steps
1. Draft schema document with required/optional fields and data types.
2. Define serde structs/enums mirroring the schema (core crate).
3. Implement validation helpers (version compatibility, required sections).
4. Document schema in `.codex/docs/project/trd.md`.
5. Prepare migration notes for future versions.

## Success Criteria
- Schema document reviewed and checked into repo.
- Schema structs compile and pass unit tests.
- Validation rejects malformed or version-mismatched payloads.
- ADR status promoted to `Complete` once schema is adopted by downstream tasks.

## Related Research
- `research/refined-research-1/analysis/persistence_recommendations.md:21`
- `research/refined-research-1/analysis/memory_models.md:3`
