# ADR-310 Git Snapshot Comparator

- **Status:** Planning
- **Task:** TASK-310
- **Owner:** Codex Memory Research Team
- **Last Updated:** 2025-10-16

## Context
Session metadata already captures git info, but Codex does not act on drift (`research/refined-research-1/analysis/codex_gap_analysis.md:14`). Detecting divergence is required before presenting cached plans.

## Decision
Implement a comparator that runs prior to each user turn:
- Gather latest `collect_git_info` snapshot.
- Compare `branch`, `head`, and `dirty` flags against cached values.
- Return a drift enum: `Aligned`, `FastForward`, `BranchSwitch`, `HistoryRewrite`, `Unknown`.
- Cache the result for downstream reconciliation logic.

## Implementation Steps
1. Expose comparator module (async-friendly).
2. Integrate into session pipeline (pre-turn guard).
3. Emit structured events/logs for drift outcomes.
4. Add unit tests covering branch switch, rebase, uncommitted changes.

## Success Criteria
- Drift accurately detected in simulated scenarios.
- False positives <5% in test suite.
- Comparator adds negligible latency (<10 ms).

## Related Research
- `research/refined-research-1/analysis/persistence_recommendations.md:39`
