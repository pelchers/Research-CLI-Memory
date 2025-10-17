# ADR-320 Reconciliation UX & CLI Flows

- **Status:** Planning
- **Task:** TASK-320
- **Owner:** Codex Memory Research Team
- **Last Updated:** 2025-10-16

## Context
Once drift is detected we must offer safe choices: fork state, rebase cache, or proceed with warnings. Without UX, caches become misleading (`research/refined-research-1/analysis/codex_gap_analysis.md:39`).

## Decision
Provide reconciliation flows:
- CLI prompt on first drift detection per session with options:
  - `fork` → duplicate cache under new branch id.
  - `rebase` → update git metadata and mark affected tasks for review.
  - `ignore` → continue with warning banner.
- TUI messaging surfaces same decisions with context.
- Record reconciliation outcome in cache for auditing.

## Implementation Steps
1. Design CLI/TUI prompts and messaging.
2. Extend `CacheManager` to support fork/rebase operations.
3. Persist reconciliation metadata (timestamp, user choice).
4. Add integration tests covering each path.

## Success Criteria
- Users can recover from branch switches without data loss.
- Cache metadata reflects reconciliation history.
- Warnings dismissed only when user explicitly chooses `ignore`.

## Related Research
- `research/refined-research-1/analysis/persistence_recommendations.md:39`
