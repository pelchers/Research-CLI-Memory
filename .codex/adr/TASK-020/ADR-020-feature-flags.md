# ADR-020 Cache Feature Flags

- **Status:** Planning
- **Task:** TASK-020
- **Owner:** Codex Memory Research Team
- **Last Updated:** 2025-10-16

## Context
The layered cache must launch behind an opt-in guard so ongoing Codex usage remains stable (`research/refined-research-1/analysis/persistence_recommendations.md:47`). Flagging enables shadow-mode deployment and gradual rollout.

## Decision
Introduce a new configuration section (`[memory.cache]`) with at least:
- `enabled` (bool) – master switch.
- `write_mode` (enum: `shadow`, `active`).
- `max_history_bytes` (optional override aligning with existing history settings).

Flags will be surfaced in CLI (`codex config`) and default to `false` (shadow writes disabled). Config errors fall back to disabled state with warnings.

## Implementation Steps
1. Extend `Config` structs and TOML parsing for `memory.cache`.
2. Wire flag checks into cache manager initialization.
3. Log effective configuration at session start.
4. Document flag usage in `.codex/docs/project/overview.md` and user docs.

## Success Criteria
- Config loads valid defaults when section absent.
- Invalid values produce warnings without crashing.
- Unit tests cover enabled/disabled/shadow scenarios.
- Documentation updated to explain rollout strategy.

## Related Research
- `research/refined-research-1/analysis/persistence_recommendations.md:47`
