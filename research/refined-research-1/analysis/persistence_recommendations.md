# Memory Persistence Strategy — Codex Focus

## Problem Statement
- **Observed gap**: Codex CLI records raw session logs (`rollout-*.jsonl`) and a global `history.jsonl`, yet nothing stitches those artefacts into a persistent project roadmap. Task progress, milestone state, and git lineage therefore reset whenever a session ends or branches change (see `analysis/codex_current_state.md`, `sources/codex/codex-rs/core/src/rollout/recorder.rs`, `.../core/src/message_history.rs`, `.../core/src/git_info.rs`).
- **Impact**: Users must re-explain project context, lose track of partially completed subtasks, and risk misaligned work after git history rewrites. Long-running repositories with multiple concurrent tasks are hit hardest.

_Reference_: Additional comparative notes on Claude Code and Gemini CLI live in `../preliminary` but are not primary drivers for this refined phase.

## User-Proposed Approaches

### A. Layered Cache (Long-Term + Short-Term)
- **Design Sketch**
  - `project_state.json` (long-term): project goals, milestones, dependency graph, latest git HEAD, links to canonical artefacts (commits, docs).
  - `sessions/<timestamp>_<branch>.json` (short-term): session summary, active subtasks, approvals, pending follow-ups.
  - On startup: load long cache → merge most relevant session file for current branch/head → hydrate model prompt preamble.
  - On shutdown: diff git state, update completion markers, append new session snapshots.
- **Strengths**
  - Deterministic file-based persistence; easy to diff/inspect via git.
  - Aligns with existing Codex infrastructure (JSONL rollouts, config).
  - Works offline and requires no external services.
- **Risks**
  - Must handle conflicts when branches diverge or users edit state manually.
  - Cache can drift if work happens outside Codex; reconciliation UX needed.
  - Requires summarisation heuristics to keep short cache compact.
- **Fit**
  - Strong candidate for Codex upstream contribution; extends current persistence layer without architectural upheaval.
  - Also portable to a bespoke Codex-derived CLI fork.

### B. Agent / Sub-agent Orchestration
- **Design Sketch**
  - Supervisor agent owns canonical task graph (project → epic → subtask).
  - Specialist agents (`planner`, `executor`, `historian`) maintain different memory scopes and consolidate into a shared datastore (e.g., SQLite + embeddings).
  - Each agent annotates outputs with git context; supervisor commits updates to `project_state.json`.
- **Strengths**
  - Enables autonomous progress tracking and proactive delegation.
  - Creates separation of concerns across planning, execution, and archival tasks.
- **Risks**
  - Higher complexity; requires robust coordination, error handling, and fallback paths.
  - Harder to upstream without sizeable changes to Codex runtime.
  - Potential latency penalties from multi-agent round-trips.
- **Fit**
  - Suited to experimentation in a Codex-inspired CLI fork; upstream work would likely need a staged rollout (start with single agent writing richer state, then optional agent plugins).

## Additional High-Leverage Options

### C. Git-Anchored Vector Memory
- **Idea**: Index embeddings of code snippets, design docs, and summarised session turns keyed by commit SHA + file path. Retrieve context using similarity + git ancestry.
- **Pros**: High recall of historical work; resilient to branching.
- **Cons**: Requires embedding store (SQLite/pgvector/Chroma); must manage model drift and storage footprint.
- **Integration Notes**: Best as an optional module complementing the layered cache rather than replacing it.

### D. Task Graph Datastore
- **Idea**: Persist a DAG of tasks with statuses, dependencies, owners, and evidence (files, commits). Expose CLI commands (`codex tasks`, `codex task complete`, `codex next`).
- **Pros**: Explicit orchestration; users can inspect, edit, and version-control the graph.
- **Cons**: Depends on disciplined updates; risk of divergence if users bypass Codex.
- **Integration**: Natural extension of the plan/update_plan concepts already surfaced in Codex.

### E. Notebook-Style Session Journals
- **Idea**: Append structured Markdown per session summarising objectives, actions, decisions, and artefacts. Link to rollouts, commits, and diffs.
- **Pros**: Human-readable audit trail; easy to review or share; leverages git history.
- **Cons**: Needs automation to avoid manual burden; may duplicate cache data if not summarised carefully.
- **Integration**: Simple optional add-on; could write to `docs/ai-journal/YYYY-MM-DD.md`.

## Evaluation Matrix

| Approach | Persistence Fidelity | Git Awareness | Implementation Effort | Upstreamability | Standalone CLI Potential | Notes |
| --- | --- | --- | --- | --- | --- | --- |
| Layered Cache | High (structured state) | Medium (head hash + file refs) | Medium | **High** | High | Requires conflict resolution + summarisation |
| Agent Orchestration | High (multi-perspective) | High (agents track git events) | High | Medium-Low (major refactor) | **Very High** | Consider staged rollout via optional module |
| Vector Memory | Medium (semantic recall) | Medium-High (commit metadata) | Medium | Medium | High | Works best paired with cache |
| Task Graph Datastore | High | High (per-node git links) | Medium | **High** | High | Aligns with existing planning concepts |
| Session Journal | Medium | Medium (manual linking) | Low | High | Medium | Improves transparency, less automation |

## Recommendations
1. **Codex Upstream Contribution**
   - Implement layered cache plus task graph datastore behind a feature flag.
   - Introduce `project_state.json` storing roadmap, git checkpoints, and backlog with CLI commands to inspect/update.
   - Extend rollout recorder to snapshot cache deltas; provide migration tooling to hydrate the cache from historical rollouts.
   - Add tests covering branch switches, rebases, detached HEAD, and manual cache edits.

2. **Codex-Derived CLI Fork**
   - Keep layered cache as the backbone; enable agent orchestration as optional module for autonomous workflows.
   - Integrate vector memory for richer retrieval, especially when coordinating multiple agents.
   - Offer import/export utilities that reuse Codex rollouts so users can migrate without losing history.

3. **Cross-Cutting Concerns**
   - **Conflict Handling**: Use git hooks or CLI pre-flight checks to detect divergence and prompt reconciliation.
   - **Summarisation Pipelines**: Automate short-cache condensation after N turns to keep prompts lean.
   - **User Control**: Provide commands to inspect, edit, or reset caches; ensure sensitive data can be pruned.
   - **Telemetry / Analytics**: Track cache hits/misses to refine heuristics and detect stale state.

## Next Actions
- Prototype schema for `project_state.json` and `sessions/*.json`.
- Design merge strategy (monotonic IDs, structured diffing, user-friendly conflict prompts).
- Explore embedding store integration for enriched retrieval (optional module).
- Document integration plan for Codex upstream vs. forked CLI implementation.
