# ADR-130 Short Cache CLI Command

- **Status:** Planning
- **Task:** TASK-130
- **Owner:** Codex Memory Research Team
- **Last Updated:** 2025-10-16

## Context
Users need an easy way to inspect recent turn memory without digging into JSON files. A CLI surface should expose the short cache summaries (`research/refined-research-1/analysis/persistence_recommendations.md:26`).

## Decision
Implement `codex cache show --recent` (and optionally `--limit N`) that:
- Reads the short cache via `CacheManager`.
- Prints human-friendly table (timestamp, task reference, summary, follow-ups).
- Provides hints when cache disabled or empty.

Command will live alongside existing CLI tooling and respect feature flags.

## Implementation Steps
1. Extend CLI argument parser with `cache` subcommand.
2. Format output for both TUI and stdout contexts.
3. Handle disabled/empty states gracefully.
4. Add integration tests (CLI smoke tests).

## Success Criteria
- Command returns within 100 ms for typical cache sizes.
- Output includes links to relevant ADR/task IDs.
- Command documented in project docs.

## Related Research
- `research/refined-research-1/analysis/codex_gap_analysis.md:26`
