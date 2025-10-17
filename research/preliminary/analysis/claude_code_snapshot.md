# Claude Code Memory & State Persistence (Observed Constraints)

- **Repository Layout**
  - Public repo focuses on plugins, hooks, and examples. Core CLI source is not present, limiting direct inspection of persistence implementation.
  - `.claude/` directory conventions appear in sample plugins (`plugins/security-guidance/hooks/security_reminder_hook.py`) and changelog entries.

- **Documented Behaviours (from CHANGELOG.md)**
  - `/memory` command allows editing of imported memory files (v1.0.94).
  - Quick-add memory shortcut (`#` prefix) documented earlier (v0.2.54).
  - `.claude/settings.json` governs shared project permissions and other settings.
  - Shell snapshots moved to `~/.claude` (v1.0.64), highlighting per-user state storage under the config dir.

- **Inferred Memory Model**
  - Memory artifacts likely reside under `~/.claude/memory/` or imported Markdown files referenced via `/memory`, with support for project-level `CLAUDE.md` (per docs references).
  - Hooks and plugins read/write state via `~/.claude/<feature>_state_<session>.json`, suggesting per-session persistence scoped to local machine.
  - Changelog references to CLAUDE.md imports (`@path/to/file.md`) imply hierarchical instruction loading similar to Codex `AGENTS.md`.

- **Open Questions**
  - Exact schema and lifecycle of `/memory` data (timestamping, git awareness, session linkage) not observable without closed-source CLI.
  - How Claude Code reconciles memory across git branches or multiple projects remains unclear.
  - Need alternative sources (docs, user reports) to confirm whether memory ties to commit hashes or purely static Markdown.

- **Action Items**
  - Parse official docs once accessible (currently HTML fetch returns 404 shell).
  - Seek community write-ups or CLI dumps describing memory file layout.
  - Consider instrumenting CLI runtime (if installed) to observe `~/.claude` structure during sessions.
