# Codex Persistence Gap Analysis

## Current Behaviour (Recap)
- **Session rollouts** capture every persisted response item in JSONL form under `~/.codex/sessions/YYYY/MM/DD/rollout-*.jsonl`. The recorder writes a `SessionMetaLine` (conversation id, cwd, CLI version, optional instructions) plus the git snapshot returned by `collect_git_info`, then streams turn data (`research/refined-research-1/sources/codex/codex-rs/core/src/rollout/recorder.rs:1` · `.../core/src/rollout/recorder.rs:223` · `.../core/src/rollout/recorder.rs:358`).
- **Global history** appends user/assistant messages to `~/.codex/history.jsonl` whenever the TUI issues `Op::AddToHistory`. Configuration supports `save-all` vs `none`, but `history.max_bytes` is currently ignored (`research/refined-research-1/sources/codex/codex-rs/core/src/message_history.rs:3` · `research/refined-research-1/sources/codex/docs/config.md:683`).
- **Conversation restart/resume** relies on replaying the raw rollout items; no higher-level synthesis is produced when sessions end or branches change (`research/refined-research-1/sources/codex/codex-rs/core/src/rollout/recorder.rs:199` · `research/refined-research-1/sources/codex/codex-rs/core/src/conversation_manager.rs:101`).
- **Planning tool** (`update_plan`) exists purely to emit UI events—payloads are not persisted or reloaded on resume, so every session starts with an empty plan (`research/refined-research-1/sources/codex/codex-rs/core/src/tools/handlers/plan.rs:47`).
- **Project docs** (`AGENTS.md`, fallbacks) are merged at session start, providing static instructions but no dynamic task state (`research/refined-research-1/sources/codex/codex-rs/core/src/project_doc.rs:34`).

## Gaps & Pitfalls
1. **No project-level state machine**  
   - Rollouts and history logs capture raw chronology but do not map work to project milestones or backlog items. There is no canonical place to record “what remains” besides ad-hoc plan entries that disappear after session termination (`research/refined-research-1/analysis/codex_current_state.md:1`).

2. **Branch/head drift is silent**  
   - `SessionMetaLine` stores git info, yet Codex never compares future turns against that snapshot. Switching branches or rebasing can leave stale context loaded without any prompt to reconcile (`research/refined-research-1/sources/codex/codex-rs/core/src/rollout/recorder.rs:223`).

3. **History size is unchecked**  
   - `history.max_bytes` is documented but unused, so `history.jsonl` grows indefinitely on active machines (`research/refined-research-1/sources/codex/docs/config.md:818`).

4. **Plans/tasks are ephemeral**  
   - The `update_plan` tool feeds the UI but the data is not written to disk; any roadmap must be reconstructed manually in each session (`research/refined-research-1/sources/codex/codex-rs/core/src/tools/handlers/plan.rs:47`).

5. **Limited summarisation**  
   - Compaction logic exists, yet Codex has no automatic “session digest” or journal tying decisions to commits. Users must inspect rollouts manually (`research/refined-research-1/sources/codex/codex-rs/core/src/rollout/recorder.rs:234`).

## Improvement Opportunities
### 1. Layered Cache (Recommended Baseline)
- **What**: Introduce `project_state.json` (long-term plan, milestones, git checkpoints) and `sessions/<timestamp>_<branch>.json` (short-term turn summaries, pending actions).
- **How**:
  - Populate cache on session bootstrap by loading the long-term state and most recent session file for the current branch/head.
  - Flush deltas after each turn; roll the short cache into long cache when tasks complete or commits land.
  - Store branch, commit hash, and working tree status so branch drift can trigger reconciliation prompts.
- **Why now**: This builds directly on existing rollout data, complements `AGENTS.md` with dynamic state, and is feasible to upstream behind a config flag (`research/refined-research-1/analysis/memory_models.md:1` · `research/refined-research-1/analysis/persistence_recommendations.md:9`).

### 2. Task Graph Datastore
- **What**: Persist a DAG (`project_tasks.yaml` or similar) mirroring the plan tool, with per-node metadata (owner, evidence, target commit).
- **Why**: Gives the CLI a structured backlog, enables `codex tasks` UX, and maps neatly onto the layered cache (long cache holds the graph, short cache tracks active node progress) (`research/refined-research-1/analysis/persistence_recommendations.md:45`).

### 3. Git-Aware Alerts
- **What**: Compare `SessionMetaLine.git` against current repo state before each turn; if mismatched, surface options to fork the cache, fast-forward, or rebuild context.
- **Why**: Prevents silent drift after rebases/branch switches while reusing existing git snapshots (`research/refined-research-1/sources/codex/codex-rs/core/src/rollout/recorder.rs:223`).

### 4. Optional Enhancements
- **Agent orchestration** for automated backlog upkeep—best explored in a fork where architectural experimentation is safer (`research/refined-research-1/analysis/persistence_recommendations.md:27`).
- **Vector memory + session journals** to improve retrieval and human audits without replacing the cache (`research/refined-research-1/analysis/persistence_recommendations.md:45`).

## Contribution vs. Fork
- **Upstream (preferred)**: Layered cache, task graph datastore, and git-drift alerts align with existing Codex architecture and can ship incrementally behind feature flags. They provide immediate value to all users without destabilising the TUI.
- **Fork**: If we want multi-agent orchestration, bespoke UX, or experimental storage backends (e.g., embeddings, remote sync), maintaining a fork gives freedom to iterate faster while still consuming upstream fixes.

## Next Steps
1. Prototype cache schema + merge strategy (conflict handling, summarisation budgets).
2. Design CLI commands/UI affordances for inspecting and updating the task graph.
3. Draft upstream proposal articulating these gaps and staged rollout (feature flag → default-on once stable).
4. Plan fork experiments only after baseline cache shows value or if upstream scope proves too disruptive.
