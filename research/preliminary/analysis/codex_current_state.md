# Codex CLI Memory Persistence (Observed)

- **Session Rollouts** (`~/.codex/sessions/YYYY/MM/DD/rollout-<timestamp>-<uuid>.jsonl`)
  - `RolloutRecorder` writes per-turn `ResponseItem`s, filtered by `is_persisted_response_item`.
  - First record is `SessionMetaLine` combining `SessionMeta` (conversation id, cwd, originator, CLI version, optional instructions, `SessionSource`) with captured `GitInfo` (commit hash, branch, repository URL).
  - Files are timestamped and stored in a calendar hierarchy for traversal and pagination (see `core/src/rollout/list.rs`).
  - Resuming sessions uses `InitialHistory::Resumed` with `RolloutRecorderParams::resume`, pointing at existing rollout files.

- **Global Message History** (`~/.codex/history.jsonl`)
  - Append-only JSONL log keyed by conversation id + timestamp; TUI issues `Op::AddToHistory` to persist notable user/assistant turns.
  - Configurable via `history.persistence` (save all vs. none); writes performed under advisory file locks to avoid interleaving.

- **Project Docs / Instructions**
  - `project_doc::read_project_docs` merges `AGENTS.md` (and fallback names like `CLAUDE.md`) across repo hierarchy into session instructions.
  - `project_doc_max_bytes` bounds ingestion; fallback filenames configurable for compatibility with other CLIs.

- **Git Awareness**
  - `collect_git_info` snapshots commit hash, branch, remote URL with 5-second timeouts, ensuring `SessionMeta` ties to repo state.
  - Additional helpers provide diff-to-remote and recent commit metadata for use in session transcripts or toolkit actions.

- **Limitations Noted**
  - Rollouts capture raw response items but require replay tooling for semantic summaries; no built-in long-term task graph or milestone tracker.
  - Git info is stored per session but not merged into a higher-level roadmap.
  - History logs are global, lacking per-repo segmentation beyond conversation `cwd`.
