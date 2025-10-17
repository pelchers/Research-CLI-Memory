# ADR-520 Migration Script

- **Status:** Planning
- **Task:** TASK-520
- **Owner:** Codex Memory Research Team
- **Last Updated:** 2025-10-16

## Context
Existing Codex users will have historical rollouts but no cache files. A migration utility is required to seed the new cache from past sessions (`research/refined-research-1/analysis/persistence_recommendations.md:143`).

## Decision
Deliver a command (`codex cache migrate --from-rollouts`) that:
- Scans `~/.codex/sessions/**/rollout-*.jsonl`.
- Extracts SessionMeta, tasks, and recent turns.
- Populates long-term/short-term cache structures.
- Produces a summary report (tasks detected, drifts flagged).

Migration runs in dry-run mode by default with optional `--apply`.

## Implementation Steps
1. Implement rollout parser leveraging existing `RolloutRecorder::get_rollout_history`.
2. Map rollout items to cache schema.
3. Handle duplicates and conflicting data.
4. Add integration test with fixture rollouts.

## Success Criteria
- Migration completes without data loss for sample history.
- Users can preview results before applying.
- Errors logged with actionable guidance.

## Related Research
- `research/refined-research-1/analysis/persistence_recommendations.md:143`
