# Working Notes

- Codex CLI (Rust) persists each conversation as `~/.codex/sessions/YYYY/MM/DD/rollout-<timestamp>-<uuid>.jsonl` with `SessionMeta` (conversation id, cwd, originator, cli version, optional instructions, `SessionSource`) plus captured `GitInfo` (commit hash, branch, remote) baked into the first record.
- Rollout files append persisted `ResponseItem`s whenever `is_persisted_response_item` returns true, and flush to disk after each update; compaction support exists via `CompactTask`.
- Global history is appended to `~/.codex/history.jsonl` when the TUI sends `Op::AddToHistory`; this log is append-only and includes timestamps + session ids for fuzzy restoration.
- `collect_git_info` runs several timed git commands to snapshot branch/commit status; support helpers exist for diffing vs remote and enumerating recent commits.
- TODO: Identify comparable mechanisms in Claude Code and Gemini CLI builds (repo scan incomplete; docs partially gated online).
- Idea: Treat git commits as checkpoints; store deltas between CLI sessions.
- Investigate: How to detect project milestones automatically from git history.
