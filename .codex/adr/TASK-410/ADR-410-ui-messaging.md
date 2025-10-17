# ADR-410 Cache UX Messaging

- **Status:** Planning
- **Task:** TASK-410
- **Owner:** Codex Memory Research Team
- **Last Updated:** 2025-10-16

## Context
Users need immediate visibility into cache status, upcoming tasks, and drift alerts. Without UI affordances the cache remains hidden (`research/refined-research-1/analysis/persistence_recommendations.md:47`).

## Decision
Enhance TUI/CLI with:
- Status banner showing cache mode (`disabled`, `shadow`, `active`).
- Active task list + next actions pulled from task graph.
- Drift warnings sourced from reconciliation module.
- Commands to toggle verbosity (`codex cache show --summary`).

Design should respect Codex TUI style and degrade gracefully in non-interactive mode.

## Implementation Steps
1. Define UI components/messages (TUI + CLI).
2. Wire cache manager events into display pipeline.
3. Provide config to silence non-critical banners.
4. Add snapshot tests for UI output.

## Success Criteria
- Cache state visible within first turn of session.
- Drift alerts remain visible until resolved.
- CLI mode prints readable summaries.

## Related Research
- `research/refined-research-1/analysis/persistence_recommendations.md:47`
