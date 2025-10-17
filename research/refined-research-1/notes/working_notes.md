# Working Notes

- Codex CLI (Rust) persists each conversation as `~/.codex/sessions/YYYY/MM/DD/rollout-<timestamp>-<uuid>.jsonl` with `SessionMeta` (conversation id, cwd, originator, cli version, optional instructions, `SessionSource`) plus captured `GitInfo` (commit hash, branch, remote) baked into the first record.
- Rollout files append persisted `ResponseItem`s whenever `is_persisted_response_item` returns true, and flush to disk after each update; compaction support exists via `CompactTask`.
- Global history is appended to `~/.codex/history.jsonl` when the TUI sends `Op::AddToHistory`; this log is append-only and includes timestamps + session ids for fuzzy restoration.
- `collect_git_info` runs several timed git commands to snapshot branch/commit status; support helpers exist for diffing vs remote and enumerating recent commits.
- Turn loop (`Session::run_task`) is the natural hook for short cache persistence; add post-processing after `record_conversation_items` to mirror everything written to history.
- Review mode stores an isolated in-memory history (`review_thread_history`); cache summaries should be emitted when `exit_review_mode` fires.
- Cache writes must be async and non-blocking; mimic `message_history::append_entry` pattern (spawn blocking I/O).
- Reference: Comparative notes for Claude Code and Gemini CLI archived under `../preliminary` if needed later.
- Idea: Treat git commits as checkpoints; store deltas between CLI sessions.
- Investigate: How to detect project milestones automatically from git history.
