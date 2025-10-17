# ADR-120 Short Cache Summarisation

- **Status:** Planning
- **Task:** TASK-120
- **Owner:** Codex Memory Research Team
- **Last Updated:** 2025-10-16

## Context
Without summarisation, per-turn cache entries could grow without bound. We need retention rules that preserve recent context while keeping files compact (`research/refined-research-1/analysis/persistence_recommendations.md:76`).

## Decision
Introduce policies for:
- Maximum number of entries per session file (e.g., 50).
- Rolling summarisation (collapse older entries into high-level notes).
- Size thresholds aligned with config (`memory.cache.max_history_bytes`).
- Optional compression or pruning for large diffs.

Policies should be configurable but ship with sensible defaults.

## Implementation Steps
1. Define retention policy struct & defaults.
2. Implement summariser (older entries → condensed summary).
3. Enforce thresholds after each append operation.
4. Add tests covering pruning, summarisation, and config overrides.

## Success Criteria
- Cache size remains below configured limit during stress tests.
- Summaries maintain key context (task id, result, follow-ups).
- Policy adjustable via config without code changes.

## Related Research
- `research/refined-research-1/analysis/memory_models.md:3`
