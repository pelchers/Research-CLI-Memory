# ADR-042 Cache Manager Blueprint

- **Status:** Complete
- **Task:** TASK-042
- **Owner:** Codex Memory Research Team
- **Last Updated:** 2025-10-18

## Context
Following schema and hook design research (ADR-010, ADR-030 planning, Phase 1 notes), we needed a detailed blueprint describing the responsibilities and API surface of the forthcoming `CacheManager` before implementation begins.

## Decision
- Define the key responsibilities: discovery of cache directories, loading/writing project state, managing short cache entries, backup rotation, and error handling.
- Draft an API skeleton (`CacheManager::load`, `update_project_state`, `append_short_cache`, etc.) and associated error types.
- Document integration points (session boot, short-cache hook, telemetry).

## Consequences
- Provides a clear roadmap for implementing TASK-030 and related subtasks.
- Highlights validation/testing requirements (backup rotation, retention pruning) ahead of coding.
- Ensures shadow-mode fallbacks and telemetry hooks are accounted for early.

## References
- `research/refined-research-1/analysis/phase1_cache_manager_blueprint.md`
- ADR-010 (Schema), ADR-030 (Cache Manager), ADR-120 (Retention Policy)
