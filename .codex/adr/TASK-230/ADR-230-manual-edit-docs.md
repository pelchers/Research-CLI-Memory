# ADR-230 Manual Edit Toolkit & Documentation

- **Status:** Planning
- **Task:** TASK-230
- **Owner:** Codex Memory Research Team
- **Last Updated:** 2025-10-16

## Context
Users and reviewers need guidance for inspecting or correcting cache data. Without docs and tooling, the cache becomes opaque (`research/refined-research-1/analysis/persistence_recommendations.md:80`).

## Decision
Provide:
- Documentation describing cache files, schema fields, and edit workflows.
- Utility command (e.g., `codex cache edit --task TASK-123`) that opens relevant sections safely.
- Guidance for resolving merge conflicts in cache JSON (git-friendly practices).

Docs live in `.codex/docs/project` and `research/` references link back to them.

## Implementation Steps
1. Draft user-facing guide covering inspection, backup, and reset steps.
2. Implement helper command to open/edit cache entries with validation.
3. Add troubleshooting section for common errors (schema mismatch, corruption).
4. Cross-link docs from ADRs and plan.

## Success Criteria
- Documentation reviewed and committed.
- Helper command validates edits before writing.
- Users can recover from simulated corruption scenarios following docs.

## Related Research
- `research/refined-research-1/analysis/persistence_recommendations.md:80`
