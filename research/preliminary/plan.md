# Research Plan

## Immediate Objectives
- Map current Codex CLI memory behaviour and identify persistence gaps.
- Evaluate long/short cache layering for project-wide continuity.
- Explore agent-based orchestration and at least two additional paradigms.

## Task Backlog
1. Document baseline: how Codex CLI stores and restores session context.
2. Draft architecture for cache-tier approach (file layout, read/write cadence, git awareness).
3. Draft agent orchestration alternative (roles, handoff protocol, state syncing).
4. Investigate other candidate strategies (e.g., vectorized embeddings, git-driven prompt synthesis).
5. Compare approaches for open-source contribution vs. bespoke CLI fork.
6. Assemble recommendation matrix with implementation risks and effort levels.

## Decision Log
- _In Progress_: Confirm primary evaluation metrics (persistence fidelity, UX friction, portability) — draft matrix in `analysis/persistence_recommendations.md`.
- _Pending_: Decide on local-only vs. optional remote sync storage.
- _Pending_: Select prototype path if user requests implementation phase.

## Notes
- All findings should tie back to repository-scoped development flows.
- Capture git state handling (branches, commits, WIP) in each proposed model.
