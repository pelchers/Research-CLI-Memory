# ADR-110 Turn Capture Hook

- **Status:** Planning
- **Task:** TASK-110
- **Owner:** Codex Memory Research Team
- **Last Updated:** 2025-10-16

## Context
To populate the short-term cache we must intercept session turns after rollout persistence occurs, gathering agent output, approvals, and git diff metadata (`research/refined-research-1/analysis/codex_gap_analysis.md:26`).

## Decision
Add a post-turn hook in the session pipeline (`Session::spawn_task` or equivalent completion path) that:
- Receives the `RolloutItem`s written for the turn.
- Summarises assistant response, approvals, command executions.
- Captures current git snapshot via `collect_git_info`.
- Writes an entry to the short cache (subject to flag state).

Hook runs asynchronously to avoid blocking the main execution flow.

## Implementation Steps
1. Identify appropriate lifecycle point after rollout flush.
2. Create aggregation logic converting rollout items → cache summary struct.
3. Invoke `CacheManager` to append to short-cache file.
4. Emit telemetry + debug logs.

## Success Criteria
- Short cache entries appear for each completed turn when flag enabled.
- No increase in perceived latency beyond target (<20 ms).
- Errors degrade gracefully (warnings, cache skipped).

## Related Research
- `research/refined-research-1/analysis/persistence_recommendations.md:76`
