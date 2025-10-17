# Research Plan

## Immediate Objectives
- Deepen understanding of Codex CLI memory behaviour and highlight outstanding persistence gaps.
- Validate long/short cache layering as a viable contribution path for Codex.
- Scope complementary models (agent orchestration, task graph, vector retrieval) specifically against Codex architecture.

## Task Backlog
1. Reconfirm Codex session persistence pipeline (rollouts, `history.jsonl`, git snapshots).
2. Elaborate cache-tier architecture (file layout, read/write cadence, git awareness) with concrete Codex integration points.
3. Outline agent orchestration alternative (roles, handoff protocol, state syncing) using Codex services.
4. Evaluate supporting strategies (vector embeddings keyed to commits, task graph datastore, journal mode).
5. Compare approaches for Codex upstream proposal vs. bespoke Codex-derived CLI.
6. Maintain recommendation matrix with implementation risks, effort, and migration considerations.

## Decision Log
- _In Progress_: Confirm evaluation metrics (persistence fidelity, UX friction, portability) — see draft matrix in `analysis/persistence_recommendations.md`.
- _Pending_: Decide on local-only vs. optional remote sync storage.
- _Pending_: Select prototype path if user requests implementation phase.

## Notes
- Scope remains Codex-first; insights from other CLIs live in `../preliminary` for reference only.
- Ensure every proposal captures git state handling (branches, commits, WIP) in the persistence story.
