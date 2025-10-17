# ADR-220 Plan Tool Integration

- **Status:** Planning
- **Task:** TASK-220
- **Owner:** Codex Memory Research Team
- **Last Updated:** 2025-10-16

## Context
The existing `update_plan` tool emits UI-only events and is filtered out of rollouts (`sources/codex/codex-rs/core/src/rollout/policy.rs:1`). To avoid losing plan context, plan updates must flow into the cache (`research/refined-research-1/analysis/codex_gap_analysis.md:20`).

## Decision
Extend the plan handler to:
- Persist plan payloads to the task graph via `CacheManager`.
- Mark relevant task nodes as `in_progress`/`completed`.
- Optionally echo plan back to the user with cache linkage.

Also provide CLI commands (`codex tasks show`, `codex tasks update`) that interact with the same data.

## Implementation Steps
1. Modify `handle_update_plan` to call cache APIs when flag enabled.
2. Map plan entries → task node updates (create if missing).
3. Implement CLI wrappers for viewing/editing plan state.
4. Add tests ensuring plan updates persist and survive restarts.

## Success Criteria
- Plans recorded in cache and surface in CLI/TUI.
- No breaking changes when cache disabled (existing behaviour preserved).
- ADR references appear in plan outputs where applicable.

## Related Research
- `research/refined-research-1/analysis/persistence_recommendations.md:35`
