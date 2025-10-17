# Memory Persistence Strategy – Comparative Analysis

## Problem Statement
- **Observed gap**: Codex CLI records raw session logs (`rollout-*.jsonl`) and a global message history but lacks a persistent, queryable representation of project objectives, task states, or git milestones. Claude Code and Gemini CLIs exhibit similar behaviour: instructions load from Markdown (e.g., `AGENTS.md`, `CLAUDE.md`) and session transcripts, yet broader project orchestration decays across sessions and branch changes.
- **Impact**: Users repeatedly re-explain project context, lose track of partially completed subtasks, and risk misaligned work after git history rewrites. Long-lived repos with multiple concurrent tasks especially suffer.

## User-Proposed Approaches

### A. Layered Cache (Long-Term + Short-Term)
- **Design Sketch**
  - `project_cache.json` (long-term): stores project goals, milestones, dependency graph, latest known git HEAD, and links to canonical artifacts (commits, docs).
  - `sessions/<timestamp>_<branch>.json` (short-term): captures session transcript summary, active subtasks, approvals, and pending follow-ups.
  - On startup: load long-term cache → merge most recent session file for current branch → hydrate model prompt.
  - On shutdown: diff git state, update completion markers, append new session snapshots.
- **Strengths**
  - Deterministic file-based persistence; easy to diff/inspect via git.
  - Aligns with existing Codex structure (JSONL logs, config).
  - Low infrastructure cost; works offline.
- **Risks**
  - Requires robust merge/conflict handling (branch switching, rebases).
  - Cache can drift from reality if user edits files outside CLI.
  - Needs summarisation heuristics to avoid ballooning prompt size.
- **Fit**
  - Strong candidate for upstream Codex contribution: extends existing persistence layer without architectural upheaval.
  - Also suitable foundation for bespoke CLI: portable, auditable.

### B. Agent / Sub-agent Orchestration
- **Design Sketch**
  - Supervisor agent maintains a canonical task graph (project > epic > subtask).
  - Specialised agents (`planner`, `executor`, `historian`) own different memory scopes and consolidate into a shared datastore (e.g., SQLite + embeddings).
  - Each agent annotates outputs with git context; supervisor commits updates to `project_state.json`.
- **Strengths**
  - Enables autonomous progress tracking and delegation.
  - Facilitates richer workflows (automatic retask, proactive alerts).
- **Risks**
  - Higher complexity; requires robust coordination and conflict resolution.
  - Harder to upstream without major architectural changes to Codex runtime.
  - Potential latency penalties; concurrency edge cases.
- **Fit**
  - Better suited for a new CLI experiment where architecture can be tailored.
  - Upstream contribution likely needs staged rollout (start with single-agent storing richer state, then optional agent plugins).

## Additional High-Leverage Options

### C. Git-Anchored Vector Memory
- **Idea**: Index embeddings of code snippets, design docs, and summarised session turns keyed by commit SHA + file path. Retrieve context using similarity + git ancestry when new tasks arrive.
- **Pros**: High recall of relevant historical work; robust to branching; supports semantic queries.
- **Cons**: Requires embedding store (SQLite/pgvector/Chroma); must handle model drift and storage footprint.
- **Integration Notes**: Could ship as optional plugin/library; complements layered cache by enriching retrieval rather than owning task state.

### D. Task Graph Datastore
- **Idea**: Persist a DAG of tasks with statuses, dependencies, owners, and evidence (files, commits). Expose CLI commands (`codex tasks`, `codex task complete`, `codex next`).
- **Pros**: Explicit orchestration; users can inspect, edit, version-control the task graph; easy to surface “what’s next”.
- **Cons**: Requires disciplined updates; risk of divergence if users bypass CLI.
- **Integration**: Natural extension of plan tool + existing `/plan` commands; can write to `project_tasks.yaml` with git-managed lifecycle.

### E. Notebook-Style Session Journals
- **Idea**: Append structured Markdown per session summarising objectives, actions, decisions, artifacts. Link to rollouts, commits, file diffs.
- **Pros**: Human-readable audit trail; easy to review or share; leverages git.
- **Cons**: Manual curation burden unless auto-summarised; may duplicate info from caches.
- **Integration**: Simple to prototype; could be optional “journal mode” that generates `docs/ai-journal/YYYY-MM-DD.md`.

## Evaluation Matrix

| Approach | Persistence Fidelity | Git Awareness | Implementation Effort | Upstreamability | Standalone CLI Potential | Notes |
| --- | --- | --- | --- | --- | --- | --- |
| Layered Cache | High (structured state) | Medium (head hash + file refs) | Medium | **High** | High | Needs conflict resolution + summarisation |
| Agent Orchestration | High (multi-perspective) | High (agents track git events) | High | Medium-Low (major refactor) | **Very High** | Consider optional plugin to minimise disruption |
| Vector Memory | Medium (semantic recall) | Medium-High (embed commit metadata) | Medium | Medium | High | Works best paired with other approaches |
| Task Graph Datastore | High | High (per-node git links) | Medium | **High** | High | Aligns with plan/update_plan concepts |
| Session Journal | Medium | Medium (manual linking) | Low | High | Medium | Good for transparency; less automation |

## Recommendations
1. **For Codex Upstream Contribution**
   - Start with the layered cache approach + task graph datastore. These map cleanly onto existing `rollout` infrastructure and can be optional via config flags.
   - Introduce a `project_state.json` (or TOML) storing project roadmap, git checkpoints, and backlog. Add CLI commands to display and update the state.
   - Extend rollout recorder to snapshot cache diffs per turn; provide migration command to build cache from historical rollouts.
   - Add tests verifying behaviour across branch switches, detached HEAD, and rebases.

2. **For a New CLI (Codex-inspired)**
   - Implement layered cache as core; embed agent orchestration as optional module to experiment with autonomous task allocation.
   - Incorporate vector memory to enhance retrieval, especially when orchestrating multiple sub-agents.
   - Provide adapters to import/export Codex/Claude/Gemini session artifacts to ease migration.

3. **Cross-Cutting Concerns**
   - **Conflict Handling**: Use git hooks or CLI pre-flight checks to detect divergence and prompt reconciliation (e.g., rebase changed tasks).
   - **Summarisation Pipelines**: Automate short-term cache condensation after N turns to keep prompts slim.
   - **User Control**: Provide commands to inspect, edit, or reset caches; ensure sensitive data can be pruned.
   - **Telemetry / Analytics**: Track cache hits/misses to refine heuristics.

## Next Actions
- Prototype schema for `project_state.json` + `sessions/*.json`.
- Design merge strategy (e.g., store history entries with monotonic IDs, allow manual conflict resolution via CLI).
- Explore embedding store for enriched retrieval (optional module).
- Document integration plan for contributing to upstream Codex vs. maintaining fork.
